//! Integration tests for the socket IPC layer.
//!
//! The daemon server uses two hardcoded paths:
//!   * `/tmp/focus_app.sock`  (socket)
//!   * `data.db`              (sqlite, created in the current working dir)
//!
//! To keep tests hermetic we run each test inside a unique temporary directory
//! (so `data.db` is isolated) but we cannot change the socket path without
//! editing the server, so all tests are forced to run **serially** via the
//! `SOCKET_LOCK` guard below and each test cleans up the socket afterwards.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

use database::detectors::toplevel::monitor::OpenAppsSnapshot;
use database::storage::repository;
use database::storage::rustqlite;

/// Guard that makes every test using the socket run one at a time.
/// (`/tmp/focus_app.sock` is a single global resource.)
static SOCKET_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn socket_lock() -> &'static Mutex<()> {
    SOCKET_LOCK.get_or_init(|| Mutex::new(()))
}

const SOCKET_PATH: &str = "/tmp/focus_app.sock";

/// A fresh, isolated working directory with an initialized `data.db`.
struct Sandbox {
    dir: PathBuf,
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl Sandbox {
    fn new() -> Self {
        // Tolerate a poisoned lock from a previously panicked test.
        let guard = match socket_lock().lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        // clear any stale socket from a previous (possibly crashed) run
        let _ = std::fs::remove_file(SOCKET_PATH);

        let dir = std::env::temp_dir().join(format!(
            "pixroot_test_{}_{}",
            std::process::id(),
            unique_id()
        ));
        std::fs::create_dir_all(&dir).expect("create sandbox dir");
        std::env::set_current_dir(&dir).expect("chdir into sandbox");
        rustqlite::init_db().expect("init db");

        Sandbox { dir, _guard: guard }
    }

    /// Start the socket server in a background thread and return its join handle.
    fn start_server(&self) -> thread::JoinHandle<()> {
        use std::os::unix::net::UnixListener;
        use std::sync::atomic::AtomicBool;
        use std::sync::Arc;
        let snapshot: OpenAppsSnapshot = Arc::new(std::sync::Mutex::new(Vec::new()));
        let active = Arc::new(AtomicBool::new(false));
        let listener = UnixListener::bind(SOCKET_PATH).expect("bind socket");
        thread::spawn(move || {
            database::ipc::socket::start_socket_server(listener, snapshot, active);
        })
    }

    /// Connect, send one JSON request, return the (first) JSON text response.
    fn request(&self, json: &str) -> String {
        let mut stream = connect_with_retry();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .ok();
        stream.write_all(json.as_bytes()).expect("write request");
        stream.flush().ok();
        // read response (server keeps connection open & streams events after,
        // so just read the first chunk, then drop the stream to disconnect).
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).expect("read response");
        String::from_utf8_lossy(&buf[..n]).to_string()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(SOCKET_PATH);
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn unique_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static C: AtomicU64 = AtomicU64::new(0);
    C.fetch_add(1, Ordering::SeqCst)
}

/// Connect to the socket, retrying briefly until the server has bound it.
fn connect_with_retry() -> UnixStream {
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        match UnixStream::connect(SOCKET_PATH) {
            Ok(s) => return s,
            Err(_) if std::time::Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(25));
            }
            Err(e) => panic!("connect to socket: {e}"),
        }
    }
}

/// Helper: sign up a user directly via the repository so we have a baseline.
fn seed_user(email: &str, username: &str, password: &str) {
    repository::sign_up(email, username, password).unwrap();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn signup_returns_ok() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    let resp = sb.request(
        r#"{"action":"signup","payload":{"email":"a@b.com","username":"alice","password":"pw123"}}"#,
    );
    assert!(resp.contains("\"action\":\"signup\""));
    assert!(resp.contains("\"status\":\"ok\""), "got: {resp}");
}

#[test]
fn signup_duplicate_email_returns_error() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    let req = r#"{"action":"signup","payload":{"email":"dup@b.com","username":"u1","password":"p"}}"#;
    let r1 = sb.request(req);
    assert!(r1.contains("\"status\":\"ok\""), "got: {r1}");

    let r2 = sb.request(req);
    assert!(r2.contains("\"status\":\"error\""), "got: {r2}");
}

#[test]
fn check_email_exists_and_not_found() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    seed_user("exists@b.com", "bob", "secret");

    let found = sb.request(r#"{"action":"check_email","payload":{"email":"exists@b.com"}}"#);
    assert!(found.contains("\"status\":\"exists\""), "got: {found}");

    let missing = sb.request(r#"{"action":"check_email","payload":{"email":"nope@b.com"}}"#);
    assert!(missing.contains("\"status\":\"not_found\""), "got: {missing}");
}

#[test]
fn login_wrong_password_and_ok() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    seed_user("login@b.com", "carol", "rightpass");

    let bad = sb.request(r#"{"action":"login","payload":{"email":"login@b.com","password":"WRONG"}}"#);
    assert!(bad.contains("\"status\":\"wrong_password\""), "got: {bad}");

    let good = sb.request(r#"{"action":"login","payload":{"email":"login@b.com","password":"rightpass"}}"#);
    assert!(good.contains("\"status\":\"ok\""), "got: {good}");
    assert!(good.contains("\"username\":\"carol\""), "got: {good}");
}

#[test]
fn login_email_not_found() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    let r = sb.request(r#"{"action":"login","payload":{"email":"ghost@b.com","password":"x"}}"#);
    assert!(r.contains("\"status\":\"email_not_found\""), "got: {r}");
}

#[test]
fn check_login_reflects_session() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    let before = sb.request(r#"{"action":"check_login","payload":{}}"#);
    assert!(before.contains("\"status\":\"bad\""), "got: {before}");

    seed_user("cli@b.com", "dave", "p");
    sb.request(r#"{"action":"login","payload":{"email":"cli@b.com","password":"p"}}"#);

    let after = sb.request(r#"{"action":"check_login","payload":{}}"#);
    assert!(after.contains("\"status\":\"ok\""), "got: {after}");
    assert!(after.contains("\"username\":\"dave\""), "got: {after}");
}

#[test]
fn start_and_end_session_ack() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    let start = sb.request(r#"{"action":"start_session","payload":{}}"#);
    assert!(start.contains("\"status\":\"ok\""), "got: {start}");

    let end = sb.request(r#"{"action":"end_session","payload":{}}"#);
    assert!(end.contains("\"status\":\"ok\""), "got: {end}");
}

#[test]
fn save_session_requires_login() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    // No user logged in -> server reports no account.
    let r = sb.request(r#"{"action":"save_session","payload":{"success":true,"time":42}}"#);
    assert!(
        r.contains("\"status\":\"no acount logined\"") || r.contains("\"status\":\"error\""),
        "got: {r}"
    );
}

#[test]
fn blocked_apps_roundtrip() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    seed_user("blk@b.com", "eve", "p");
    sb.request(r#"{"action":"login","payload":{"email":"blk@b.com","password":"p"}}"#);

    let set = sb.request(r#"{"action":"set_blocked_apps","payload":{"blocked_apps":"firefox,discord"}}"#);
    assert!(set.contains("\"status\":\"ok\""), "got: {set}");

    let get = sb.request(r#"{"action":"get_blocked_apps","payload":{}}"#);
    assert!(get.contains("\"blocked_apps\":\"firefox,discord\""), "got: {get}");
}

#[test]
fn warning_roundtrip() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    seed_user("warn@b.com", "frank", "p");
    sb.request(r#"{"action":"login","payload":{"email":"warn@b.com","password":"p"}}"#);

    let set = sb.request(r#"{"action":"set_warning","payload":{"warning":true}}"#);
    assert!(set.contains("\"status\":\"ok\""), "got: {set}");

    let get = sb.request(r#"{"action":"get_warning","payload":{}}"#);
    assert!(get.contains("\"warning\":true"), "got: {get}");
}

#[test]
fn all_apps_returns_list() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    let r = sb.request(r#"{"action":"all_apps","payload":{}}"#);
    assert!(r.contains("\"action\":\"all_apps\""), "got: {r}");
    assert!(r.contains("\"status\":\"ok\""), "got: {r}");
    assert!(r.contains("\"all_apps\""), "got: {r}");
}

#[test]
fn log_out_clears_session() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    seed_user("lo@b.com", "grace", "p");
    sb.request(r#"{"action":"login","payload":{"email":"lo@b.com","password":"p"}}"#);

    let logged_in = sb.request(r#"{"action":"check_login","payload":{}}"#);
    assert!(logged_in.contains("\"status\":\"ok\""), "got: {logged_in}");

    let out = sb.request(r#"{"action":"log_out","payload":{}}"#);
    assert!(out.contains("\"status\":\"ok\""), "got: {out}");

    let after = sb.request(r#"{"action":"check_login","payload":{}}"#);
    assert!(after.contains("\"status\":\"bad\""), "got: {after}");
}

#[test]
fn unknown_action_is_ignored() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    // Server prints "unknown action" and writes nothing; the connection is
    // still kept open and streams no events, so the read will time out.
    let mut stream = connect_with_retry();
    stream.set_read_timeout(Some(Duration::from_millis(300))).ok();
    stream
        .write_all(b"{\"action\":\"frobnicate\",\"payload\":{}}")
        .unwrap();
    stream.flush().ok();
    let mut buf = [0u8; 64];
    let n = stream.read(&mut buf).unwrap_or(0);
    assert_eq!(n, 0, "server should not respond to unknown actions");
}

#[test]
fn malformed_json_is_rejected() {
    let sb = Sandbox::new();
    let _server = sb.start_server();

    let mut stream = connect_with_retry();
    stream.set_read_timeout(Some(Duration::from_millis(300))).ok();
    stream.write_all(b"not json at all").unwrap();
    stream.flush().ok();
    let mut buf = [0u8; 64];
    let n = stream.read(&mut buf).unwrap_or(0);
    assert_eq!(n, 0, "server should ignore unparseable requests");
}
