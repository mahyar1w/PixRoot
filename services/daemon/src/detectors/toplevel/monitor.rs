use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::detectors::toplevel::apps;

/// Latest snapshot of currently open GUI apps, shared with client handler threads.
pub type OpenAppsSnapshot = Arc<Mutex<Vec<String>>>;

/// Polls `open_apps()` every `interval` and overwrites `snapshot` with the result.
/// Runs on its own thread; client handlers read `snapshot` to detect newly opened
/// (and blocked) apps and emit `interrupt` events over IPC.
pub fn start_monitor(interval: Duration, snapshot: OpenAppsSnapshot) {
    thread::spawn(move || loop {
        let open = apps::open_apps().into_iter().collect();
        *snapshot.lock().unwrap() = open;
        thread::sleep(interval);
    });
}
