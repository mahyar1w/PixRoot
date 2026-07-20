use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    database::storage::rustqlite::init_db().unwrap();

    let snapshot = Arc::new(Mutex::new(Vec::<String>::new()));
    let session_active = Arc::new(AtomicBool::new(false));
    database::detectors::toplevel::monitor::start_monitor(
        Duration::from_secs(3),
        snapshot.clone(),
        session_active.clone(),
    );

    database::ipc::socket::start_socket_server(snapshot, session_active);
}
