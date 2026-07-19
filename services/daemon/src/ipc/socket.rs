use std::collections::BTreeSet;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
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
pub fn start_socket_server(snapshot: OpenAppsSnapshot) {
    let _ = fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH).expect("bind socket");
    println!("socket started");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let snapshot = Arc::clone(&snapshot);
                thread::spawn(move || handle_client(stream, snapshot));
            }
            Err(e) => eprintln!("accept error: {}", e),
        }
    }
}

fn handle_client(mut stream: UnixStream, snapshot: OpenAppsSnapshot) {
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
                        Ok(request) => handle_action(&msg, &request.action, &mut stream),
                        Err(e) => eprintln!("bad request: {}", e),
                    }
                    inbound = rest;
                }
            }
            Err(_) => {} // timeout / would-block: no new data this tick
        }

        // (3) Stream `interrupt` events for newly blocked apps.
        thread::sleep(Duration::from_millis(1200));
        let open: BTreeSet<String> = snapshot.lock().unwrap().iter().cloned().collect();
        let newly_opened: Vec<String> = open.difference(&last_open).cloned().collect();
        last_open = open;

        if newly_opened.is_empty() {
            continue;
        }

        if let Some(blocked) = blocked_apps_for_logged_in_user() {
            if blocked.is_empty() {
                continue;
            }
            let matched: Vec<&String> =
                newly_opened.iter().filter(|b| blocked.contains(*b)).collect();
            if matched.is_empty() {
                continue;
            }

            let warning = warning_for_logged_in_user().unwrap_or(false);
            let stage = if warning {
                "warning_failed"
            } else {
                "interrupted"
            };
            let event = Interrupt {
                event: "interrupt".to_string(),
                payload: crate::ipc::struct_ser_to_cl::InterruptPayload {
                    warning,
                    stage: stage.to_string(),
                },
            };
            if let Ok(json) = serde_json::to_string(&event) {
                if stream.write_all(json.as_bytes()).is_err() {
                    break;
                }
            }
        }
    }
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

fn blocked_apps_for_logged_in_user() -> Option<std::collections::HashSet<String>> {
    let conn = Connection::open(db_path()).ok()?;
    let raw = repository::get_blocked_apps(&conn).ok()?;
    if raw.trim().is_empty() {
        return Some(Default::default());
    }
    Some(raw.split(',').map(|s| s.trim().to_string()).collect())
}

fn warning_for_logged_in_user() -> Option<bool> {
    let conn = Connection::open(db_path()).ok()?;
    repository::get_warning(&conn).ok()
}

fn handle_action(buffer: &str, action: &str, stream: &mut UnixStream) {
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
