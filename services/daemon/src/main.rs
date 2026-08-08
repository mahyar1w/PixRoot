use std::io::ErrorKind;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SOCKET_PATH: &str = "/tmp/focus_app.sock";

/// Returns true if a live daemon is already accepting on the socket path.
/// A successful connect means another instance owns the socket. A failure
/// (no file, or stale file with no listener) means it is safe for us to bind.
fn live_daemon_exists() -> bool {
    match UnixStream::connect(SOCKET_PATH) {
        Ok(stream) => {
            drop(stream);
            true
        }
        Err(e) if e.kind() == ErrorKind::ConnectionRefused => false,
        Err(_) => false,
    }
}

fn main() {
    // Single-instance handling. If another daemon is alive and accepting on the
    // socket, exit quietly. Otherwise remove any stale socket file (leftover
    // from a previous run whose daemon died without the file being unlinked)
    // and bind a fresh listener for the program lifetime.
    if live_daemon_exists() {
        eprintln!("Another daemon instance is already running, exiting");
        std::process::exit(0);
    }

    std::fs::remove_file(SOCKET_PATH).ok();

    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind socket: {}", e);
            std::process::exit(1);
        }
    };

    database::storage::rustqlite::init_db().unwrap();

    let snapshot = Arc::new(Mutex::new(Vec::<String>::new()));
    let session_active = Arc::new(AtomicBool::new(false));
    database::detectors::toplevel::monitor::start_monitor(
        Duration::from_secs(3),
        snapshot.clone(),
        session_active.clone(),
    );

    database::ipc::socket::start_socket_server(listener, snapshot, session_active);

    // The server loop only returns once the last client disconnected; unlink
    // the socket so the next launch can bind a fresh one.
    let _ = std::fs::remove_file(SOCKET_PATH);
}
