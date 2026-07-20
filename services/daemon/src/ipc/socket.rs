use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rusqlite::Connection;

use crate::detectors::toplevel::apps;
use crate::detectors::toplevel::monitor::OpenAppsSnapshot;
use crate::ipc::struct_cl_to_ser::BaseRequest;
use crate::ipc::struct_ser_to_cl::Interrupt;
use crate::storage::repository;
use crate::storage::rustqlite::db_path;

const SOCKET_PATH: &str = "/tmp/focus_app.sock";

/// `snapshot` is the shared, periodically updated list of currently open apps.
/// Each client connection keeps the socket open after the request and streams
/// `interrupt` events whenever a newly opened app matches the blocked list.
pub fn start_socket_server(snapshot: OpenAppsSnapshot, active: Arc<AtomicBool>) {
    let _ = fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH).expect("bind socket");
    println!("socket started");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let snapshot = Arc::clone(&snapshot);
                let active = Arc::clone(&active);
                thread::spawn(move || handle_client(stream, snapshot, active));
            }
            Err(e) => eprintln!("accept error: {}", e),
        }
    }
}

fn handle_client(mut stream: UnixStream, snapshot: OpenAppsSnapshot, active: Arc<AtomicBool>) {
    // Non-blocking reads: each connection may send multiple JSON messages over
    // its lifetime, so we poll for new inbound bytes and parse complete messages
    // as they arrive (see `try_parse_message`).
    stream
        .set_read_timeout(Some(std::time::Duration::from_millis(200)))
        .ok();

    // Accumulated inbound bytes not yet consumed as a complete JSON message.
    let mut inbound: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 1024];

    // Latest set of open apps, used to detect newly opened (blocked) apps.
    let mut last_open: BTreeSet<String> = Default::default();

    // Active warning grace period, if any. When a blocked app opens while
    // warning mode is on, we emit `warning_successed` and start a 3s grace: the
    // user must close the offending app(s) before it expires. Holds the set of
    // offending app names and the instant the grace began.
    let mut grace: Option<(std::collections::HashSet<String>, std::time::Instant)> = None;

    // Cached installed-apps map (exec-bin -> display name), rebuilt on
    // `start_session`. Used to bridge process names and blocked display names.
    let mut installed: HashMap<String, String> = HashMap::new();

    loop {
        // (1) Read any new inbound bytes (non-blocking via the read timeout).
        match stream.read(&mut chunk) {
            Ok(0) => break, // client disconnected
            Ok(n) => {
                inbound.extend_from_slice(&chunk[..n]);
                // (2) Dispatch every complete JSON message currently buffered.
                while let Some((msg, rest)) = try_parse_message(&inbound) {
                    println!("received: {}", msg);
                    match serde_json::from_str::<BaseRequest>(&msg) {
                        Ok(request) => handle_action(
                            &msg,
                            &request.action,
                            &mut stream,
                            &mut grace,
                            &active,
                            &mut installed,
                        ),
                        Err(e) => eprintln!("bad request: {}", e),
                    }
                    inbound = rest;
                }
            }
            Err(_) => {} // timeout / would-block: no new data this tick
        }

        // (3) Stream `interrupt` events for newly blocked apps.
        thread::sleep(Duration::from_millis(1200));

        // Only run interrupt detection while a session is active. When inactive,
        // reset per-session state so the next session starts clean.
        if !active.load(Ordering::Relaxed) {
            grace = None;
            last_open.clear();
            continue;
        }

        let open: BTreeSet<String> = snapshot.lock().unwrap().iter().cloned().collect();
        let newly_opened: Vec<String> = open.difference(&last_open).cloned().collect();
        last_open = open.clone();

        let warning = warning_for_logged_in_user().unwrap_or(false);
        let blocked = blocked_apps_for_logged_in_user().unwrap_or_default();

        // (3a) Resolve any active grace period first. We only decide at the 3s
        // deadline: if the offending app is still open -> `warning_failed`,
        // otherwise (closed in time) -> `warning_successed`.
        if let Some((offending, started)) = grace.clone() {
            if started.elapsed() >= Duration::from_secs(3) {
                let still_open = offending.iter().any(|a| open.contains(a));
                if still_open {
                    println!(
                        "interrupt: grace expired, still open={:?} -> stage=warning_failed",
                        offending
                    );
                    emit_interrupt(&mut stream, true, "warning_failed");
                } else {
                    println!(
                        "interrupt: grace expired, offending app(s) closed -> stage=warning_successed"
                    );
                    emit_interrupt(&mut stream, true, "warning_successed");
                }
                grace = None;
            }
            // While a grace is active we do not start another one.
            continue;
        }

        if newly_opened.is_empty() || blocked.is_empty() {
            continue;
        }

        let matched: Vec<String> = newly_opened
            .iter()
            .filter(|b| apps::matches_blocked(b, &blocked, &installed))
            .cloned()
            .collect();
        if matched.is_empty() {
            continue;
        }

        if !warning {
            // Warning mode off: any blocked app opening is an immediate
            // interruption (event #1).
            println!(
                "interrupt: blocked app matched={:?} warning=false -> stage=interrupted",
                matched
            );
            emit_interrupt(&mut stream, false, "interrupted");
        } else {
            // Warning mode on: emit the initial warning (`interrupted`) at t=0
            // and start a 3s grace. The outcome (success/fail) is decided when
            // the grace deadline elapses (see 3a).
            println!(
                "interrupt: blocked app matched={:?} warning=true -> stage=interrupted (grace started)",
                matched
            );
            emit_interrupt(&mut stream, true, "interrupted");
            grace = Some((matched.into_iter().collect(), std::time::Instant::now()));
        }
    }
}

/// Serialize and send a newline-terminated `interrupt` event to the client.
fn emit_interrupt(stream: &mut UnixStream, warning: bool, stage: &str) {
    let event = Interrupt {
        event: "interrupt".to_string(),
        payload: crate::ipc::struct_ser_to_cl::InterruptPayload {
            warning,
            stage: stage.to_string(),
        },
    };
    println!("interrupt: emitting event warning={} stage={}", warning, stage);
    send(stream, &event);
}

/// Try to parse one complete JSON message from the front of `buf`.
///
/// Returns `(parsed_json_string, remaining_bytes)` on success, else `None`.
/// Non-destructive: partial/unparseable data is left in `buf` for a later call.
/// Messages are raw JSON with no delimiter (the client writes one complete
/// object per request), so we parse a `Value` and re-serialize it to recover
/// the exact consumed byte length.
fn try_parse_message(buf: &[u8]) -> Option<(String, Vec<u8>)> {
    let pos = buf.iter().position(|&b| b == b'\n')?;
    let msg_bytes = &buf[..pos];
    let s = std::str::from_utf8(msg_bytes).ok()?;

    let _value: serde_json::Value = serde_json::from_str(s).ok()?;

    let rest = buf[pos + 1..].to_vec();
    Some((s.to_string(), rest))
}

/// Returns the blocked-app names for the logged-in user, normalized for tolerant
/// matching (see `apps::normalize`). Empty/blank entries are dropped.
fn blocked_apps_for_logged_in_user() -> Option<std::collections::HashSet<String>> {
    let conn = Connection::open(db_path()).ok()?;
    let raw = repository::get_blocked_apps(&conn).ok()?;
    if raw.trim().is_empty() {
        return Some(Default::default());
    }
    Some(
        raw.split(',')
            .map(|s| apps::normalize(s))
            .filter(|s| !s.is_empty())
            .collect(),
    )
}

fn warning_for_logged_in_user() -> Option<bool> {
    let conn = Connection::open(db_path()).ok()?;
    repository::get_warning(&conn).ok()
}

fn handle_action(
    buffer: &str,
    action: &str,
    stream: &mut UnixStream,
    grace: &mut Option<(std::collections::HashSet<String>, std::time::Instant)>,
    active: &Arc<AtomicBool>,
    installed: &mut HashMap<String, String>,
) {
    match action {
        "check_email" => {
            let req: crate::ipc::struct_cl_to_ser::CheckEmail =
                serde_json::from_str(buffer).unwrap();
            let conn = Connection::open(db_path()).unwrap();

            match repository::check_email(&conn, &req.payload.email) {
                Ok(Some(_user)) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::CheckEmail {
                        action: "check_email".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::CheckEmailPayload {
                            status: "ok".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Ok(None) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::CheckEmail {
                        action: "check_email".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::CheckEmailPayload {
                            status: "bad".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::CheckEmail {
                        action: "check_email".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::CheckEmailPayload {
                            status: "error".to_string(),
                            error: e.to_string(),
                        },
                    },
                ),
            }
        }

        "signup" => {
            let req: crate::ipc::struct_cl_to_ser::SignUp =
                serde_json::from_str(buffer).unwrap();
            match repository::sign_up(
                &req.payload.email,
                &req.payload.username,
                &req.payload.password,
            ) {
                Ok(_) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::SignUp {
                        action: "signup".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::SignUpPayload {
                            status: "ok".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::SignUp {
                        action: "signup".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::SignUpPayload {
                            status: "error".to_string(),
                            error: e.to_string(),
                        },
                    },
                ),
            }
        }

        "login" => {
            let req: crate::ipc::struct_cl_to_ser::Login =
                serde_json::from_str(buffer).unwrap();
            let conn = Connection::open(db_path()).unwrap();
            match repository::check_email(&conn, &req.payload.email) {
                Ok(Some(user)) => match repository::check_password(&user, &req.payload.password) {
                    Ok(true) => {
                        repository::set_login(&conn, &user.user_id).unwrap();
                        send(
                            stream,
                            crate::ipc::struct_ser_to_cl::Login {
                                action: "login".to_string(),
                                payload: crate::ipc::struct_ser_to_cl::LoginPayload {
                                    status: "ok".to_string(),
                                    username: user.username.clone(),
                                    error: "".to_string(),
                                },
                            },
                        );
                    }
                    Ok(false) => send(
                        stream,
                        crate::ipc::struct_ser_to_cl::Login {
                            action: "login".to_string(),
                            payload: crate::ipc::struct_ser_to_cl::LoginPayload {
                                status: "wrong_password".to_string(),
                                username: "".to_string(),
                                error: "".to_string(),
                            },
                        },
                    ),
                    Err(e) => send(
                        stream,
                        crate::ipc::struct_ser_to_cl::Login {
                            action: "login".to_string(),
                            payload: crate::ipc::struct_ser_to_cl::LoginPayload {
                                status: "error".to_string(),
                                username: "".to_string(),
                                error: e.to_string(),
                            },
                        },
                    ),
                },
                Ok(None) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::Login {
                        action: "login".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::LoginPayload {
                            status: "email_not_found".to_string(),
                            username: "".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::Login {
                        action: "login".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::LoginPayload {
                            status: "error".to_string(),
                            username: "".to_string(),
                            error: e.to_string(),
                        },
                    },
                ),
            }
        }

        "check_login" => {
            println!("check login request");
            let conn = Connection::open(db_path()).unwrap();
            match repository::get_logged_in_user(&conn) {
                Ok(Some(user)) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::CheckLogin {
                        action: "check_login".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::CheckLoginPayload {
                            status: "ok".to_string(),
                            username: user.username,
                            error: "".to_string(),
                        },
                    },
                ),
                Ok(None) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::CheckLogin {
                        action: "check_login".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::CheckLoginPayload {
                            status: "bad".to_string(),
                            username: "".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::CheckLogin {
                        action: "check_login".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::CheckLoginPayload {
                            status: "error".to_string(),
                            username: "".to_string(),
                            error: e.to_string(),
                        },
                    },
                ),
            }
        }

        "save_session" => {
            let req: crate::ipc::struct_cl_to_ser::SaveSession =
            serde_json::from_str(buffer).unwrap();

            let conn = Connection::open(db_path()).unwrap();

        match repository::get_logged_in_user(&conn) {

            Ok(Some(user)) => {

                match repository::register_attempt(
                    &conn,
                    &user.user_id,
                    req.payload.success,
                    req.payload.time,
            ) {

                Ok(_) => {

                    send(
                        stream,
                        crate::ipc::struct_ser_to_cl::SaveSession {
                            action: "save_session".to_string(),
                            payload: crate::ipc::struct_ser_to_cl::SaveSessionPayload {
                                status: "ok".to_string(),
                                error: "".to_string(),
                            },
                        },
                    );
                    println!("before get_total_day_time");

                    let result=repository::get_total_day_time(
                        &conn,
                        &user.user_id,
                    );
                    println!("after get_total_day_time");
                    match result {

                        Ok(total_time) => {
                            println!("sending total day time");
                            send(
                                stream,
                                crate::ipc::struct_ser_to_cl::TotalDayTime {
                                    event: "total_day_time".to_string(),
                                    payload: crate::ipc::struct_ser_to_cl::TotalDayTimePayload {
                                        status: "ok".to_string(),
                                        total_focus: total_time,
                                        error: "".to_string(),
                                    },
                                },
                            );
                            println!("event send successfully");
                        }

                        Err(e) => {
                            
                            send(
                                stream,
                                crate::ipc::struct_ser_to_cl::TotalDayTime {
                                    event: "total_day_time".to_string(),
                                    payload: crate::ipc::struct_ser_to_cl::TotalDayTimePayload {
                                        status: "bad".to_string(),
                                        total_focus: 0,
                                        error: e.to_string(),
                                    },
                                },
                            );
                        }
                    }
                }

                Err(e) => {
                    send(
                        stream,
                        crate::ipc::struct_ser_to_cl::SaveSession {
                            action: "save_session".to_string(),
                            payload: crate::ipc::struct_ser_to_cl::SaveSessionPayload {
                                status: "error".to_string(),
                                error: e.to_string(),
                            },
                        },
                    );
                }
            }
        }

        Ok(None) => {
            send(
                stream,
                crate::ipc::struct_ser_to_cl::SaveSession {
                    action: "save_session".to_string(),
                    payload: crate::ipc::struct_ser_to_cl::SaveSessionPayload {
                        status: "no account logged".to_string(),
                        error: "".to_string(),
                    },
                },
            );
        }

        Err(e) => {
            send(
                stream,
                crate::ipc::struct_ser_to_cl::SaveSession {
                    action: "save_session".to_string(),
                    payload: crate::ipc::struct_ser_to_cl::SaveSessionPayload {
                        status: "error".to_string(),
                        error: e.to_string(),
                    },
                },
            );
        }
    }
}

        "start_session" => {
            // Fresh session: clear any lingering grace state from a prior one,
            // start the app scanner, and refresh the installed-apps cache.
            *grace = None;
            *installed = apps::installed_apps();
            active.store(true, Ordering::Relaxed);
            send(
                stream,
                crate::ipc::struct_ser_to_cl::StartSession {
                    action: "start_session".to_string(),
                    payload: crate::ipc::struct_ser_to_cl::StartSessionPayload {
                        status: "ok".to_string(),
                        error: "".to_string(),
                    },
                },
            );
        }

        "end_session" => {
            // Session over: stop the app scanner and clear grace state.
            active.store(false, Ordering::Relaxed);
            *grace = None;
            send(
                stream,
                crate::ipc::struct_ser_to_cl::EndSession {
                    action: "end_session".to_string(),
                    payload: crate::ipc::struct_ser_to_cl::EndSessionPayload {
                        status: "ok".to_string(),
                        error: "".to_string(),
                    },
                },
            );
        }

        "all_apps" => {
            let installed = apps::installed_apps();
            let names: Vec<String> = installed.into_values().collect();
            send(
                stream,
                crate::ipc::struct_ser_to_cl::AllApps {
                    action: "all_apps".to_string(),
                    payload: crate::ipc::struct_ser_to_cl::AllAppsPayload {
                        status: "ok".to_string(),
                        all_apps: names,
                        error: "".to_string(),
                    },
                },
            );
        }

        "set_blocked_apps" => {
            let req: crate::ipc::struct_cl_to_ser::SetBlockedApp =
                serde_json::from_str(buffer).unwrap();
            let conn = Connection::open(db_path()).unwrap();
            match repository::set_blocked_apps(&conn, &req.payload.blocked_apps) {
                Ok(_) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::SetBlockedApp {
                        action: "set_blocked_apps".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::SetBlockedAppPayload {
                            status: "ok".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::SetBlockedApp {
                        action: "set_blocked_apps".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::SetBlockedAppPayload {
                            status: "error".to_string(),
                            error: e.to_string(),
                        },
                    },
                ),
            }
        }

        "get_blocked_apps" => {
            let conn = Connection::open(db_path()).unwrap();
            match repository::get_blocked_apps(&conn) {
                Ok(blocked) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::GetBlockedApp {
                        action: "get_blocked_apps".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::GetBlockedAppPayload {
                            blocked_apps: blocked,
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::GetBlockedApp {
                        action: "get_blocked_apps".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::GetBlockedAppPayload {
                            blocked_apps: format!("error:{}", e),
                        },
                    },
                ),
            }
        }

        "set_warning" => {
            let req: crate::ipc::struct_cl_to_ser::SetWarning =
                serde_json::from_str(buffer).unwrap();
            let conn = Connection::open(db_path()).unwrap();
            match repository::set_warning(&conn, req.payload.warning) {
                Ok(_) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::SetWarning {
                        action: "set_warning".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::SetWarningPayload {
                            status: "ok".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::SetWarning {
                        action: "set_warning".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::SetWarningPayload {
                            status: "error".to_string(),
                            error: e.to_string(),
                        },
                    },
                ),
            }
        }

        "get_warning" => {
            let conn = Connection::open(db_path()).unwrap();
            match repository::get_warning(&conn) {
                Ok(warning) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::GetWarning {
                        action: "get_warning".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::GetWarningPayload { warning },
                    },
                ),
                Err(_) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::GetWarning {
                        action: "get_warning".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::GetWarningPayload {
                            warning: false,
                        },
                    },
                ),
            }
        }

        "log_out" => {
            let conn = Connection::open(db_path()).unwrap();
            match repository::get_logged_in_user(&conn) {
                Ok(Some(user)) => match repository::logout(&conn, &user.user_id) {
                    Ok(_) => send(
                        stream,
                        crate::ipc::struct_ser_to_cl::LogOut {
                            action: "log_out".to_string(),
                            payload: crate::ipc::struct_ser_to_cl::LogOutPayload {
                                status: "ok".to_string(),
                                error: "".to_string(),
                            },
                        },
                    ),
                    Err(e) => send(
                        stream,
                        crate::ipc::struct_ser_to_cl::LogOut {
                            action: "log_out".to_string(),
                            payload: crate::ipc::struct_ser_to_cl::LogOutPayload {
                                status: "error".to_string(),
                                error: e.to_string(),
                            },
                        },
                    ),
                },
                Ok(None) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::LogOut {
                        action: "log_out".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::LogOutPayload {
                            status: "bad".to_string(),
                            error: "".to_string(),
                        },
                    },
                ),
                Err(e) => send(
                    stream,
                    crate::ipc::struct_ser_to_cl::LogOut {
                        action: "log_out".to_string(),
                        payload: crate::ipc::struct_ser_to_cl::LogOutPayload {
                            status: "error".to_string(),
                            error: e.to_string(),
                        },
                    },
                ),
            }
        }

        _ => println!("unknown action: {}", action),
    }
}

fn send<T: serde::Serialize>(stream: &mut UnixStream, value: T) {
    if let Ok(json) = serde_json::to_string(&value) {
        let message = format!("{}\n", json);

        let _ = stream.write_all(message.as_bytes());
        let _ = stream.flush();
    }
}
