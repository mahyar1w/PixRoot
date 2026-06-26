use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};


fn get_installed_gui_apps() -> HashMap<String, String> {
    let mut registry = HashMap::new();
    let paths = vec![
        PathBuf::from("/usr/share/applications"),
        dirs::home_dir().map(|h| h.join(".local/share/applications")).unwrap_or_default(),
    ];

    for path in paths {
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("desktop") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    let mut name = None;
                    let mut exec_bin = None;
                    let mut is_hidden = false;

                    for line in content.lines() {
                        if line.contains("NoDisplay=true") || line.contains("Hidden=true") {
                            is_hidden = true;
                        }
                        if line.starts_with("Name=") && name.is_none() {
                            name = Some(line[5..].trim().to_string());
                        }
                        if line.starts_with("Exec=") && exec_bin.is_none() {
                            let cmd = line[5..].split_whitespace().next().unwrap_or("");
                            exec_bin = Path::new(cmd).file_name().and_then(|s| s.to_str()).map(|s| s.to_string());
                        }
                    }

                    if !is_hidden {
                        if let (Some(n), Some(e)) = (name, exec_bin) {
                            registry.insert(e, n);
                        }
                    }
                }
            }
        }
    }
    registry
}


fn get_open_gui_apps() -> BTreeSet<String> {
    let mut open_bins = BTreeSet::new();
    let uid = unsafe { libc::getuid() };

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() { continue; }

            if fs::metadata(&path).map(|m| m.uid()).unwrap_or(0) != uid { continue; }

            let pid_str = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if !pid_str.chars().all(|c| c.is_ascii_digit()) { continue; }

            let env = fs::read(path.join("environ")).unwrap_or_default();
            let env_str = String::from_utf8_lossy(&env);
            if !env_str.contains("WAYLAND_DISPLAY=") && !env_str.contains("DISPLAY=") {
                continue;
            }

            let cmdline = fs::read(path.join("cmdline")).unwrap_or_default();
            if let Some(first_part) = cmdline.split(|&b| b == 0).next() {
                let full_path = String::from_utf8_lossy(first_part);
                if let Some(bin_name) = Path::new(&*full_path).file_name().and_then(|s| s.to_str()) {
                    
                    let blacklist = [
                        "gsd-", "gvfs", "ibus", "at-spi", "dbus", "xdg-", "gnome-shell",
                        "Xwayland", "dconf", "evolution", "goa-", "localsearch", "mutter",
                        "bash", "cat", "python", "gjs", "java", "bwrap", "chrome_crashpad",
                        "database", "rust-analyzer", "obexd", "exe"
                    ];

                    if !blacklist.iter().any(|&s| bin_name.contains(s)) {
                        open_bins.insert(bin_name.to_string());
                    }
                }
            }
        }
    }
    open_bins
}



fn main() {
    
    let installed_registry = get_installed_gui_apps();
    let open_binaries = get_open_gui_apps();

}
