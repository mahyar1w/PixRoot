use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::detectors::toplevel::apps;

/// Latest snapshot of currently open GUI apps, shared with client handler threads.
pub type OpenAppsSnapshot = Arc<Mutex<Vec<String>>>;

/// Polls `open_apps()` every `interval` and overwrites `snapshot` with the result,
/// but only while a session is active (`active` is true). When no session is
/// active the snapshot is cleared and scanning is skipped.
/// Runs on its own thread; client handlers read `snapshot` to detect newly opened
/// (and blocked) apps and emit `interrupt` events over IPC.
pub fn start_monitor(interval: Duration, snapshot: OpenAppsSnapshot, active: Arc<AtomicBool>) {
    thread::spawn(move || loop {
        if !active.load(Ordering::Relaxed) {
            snapshot.lock().unwrap().clear();
            thread::sleep(interval);
            continue;
        }

        let open: Vec<String> = apps::open_apps().into_iter().collect();
        println!("open apps: {:?}", open);
        *snapshot.lock().unwrap() = open;
        thread::sleep(interval);
    });
}
