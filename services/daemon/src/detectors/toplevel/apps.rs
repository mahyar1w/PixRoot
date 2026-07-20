use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

pub fn installed_apps() -> HashMap<String, String> {
    let mut registry = HashMap::new();
    let paths = vec![
        PathBuf::from("/usr/share/applications"),
        dirs::home_dir()
            .map(|h| h.join(".local/share/applications"))
            .unwrap_or_default(),
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
                            exec_bin = Path::new(cmd)
                                .file_name()
                                .and_then(|s| s.to_str())
                                .map(|s| s.to_string());
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

pub fn open_apps() -> BTreeSet<String> {
    let mut open_bins = BTreeSet::new();
    let uid = unsafe { libc::getuid() };

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            if fs::metadata(&path).map(|m| m.uid()).unwrap_or(0) != uid {
                continue;
            }

            let pid_str = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if !pid_str.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            let env = fs::read(path.join("environ")).unwrap_or_default();
            let env_str = String::from_utf8_lossy(&env);
            if !env_str.contains("WAYLAND_DISPLAY=") && !env_str.contains("DISPLAY=") {
                continue;
            }

            let cmdline = fs::read(path.join("cmdline")).unwrap_or_default();
            if let Some(first_part) = cmdline.split(|&b| b == 0).next() {
                let full_path = String::from_utf8_lossy(first_part);
                // Ignore Wine / Windows apps (paths like `C:\...\foo.exe`).
                if is_wine_app(&full_path) {
                    continue;
                }
                if let Some(bin_name) =
                    Path::new(&*full_path).file_name().and_then(|s| s.to_str())
                {
                    let blacklist = [
                        "gsd-", "gvfs", "ibus", "at-spi", "dbus", "xdg-", "gnome-shell",
                        "Xwayland", "dconf", "evolution", "goa-", "localsearch", "mutter",
                        "bash", "cat", "python", "gjs", "java", "bwrap", "chrome_crashpad",
                        "database", "rust-analyzer", "obexd", "exe",
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

/// True for Wine / Windows executables, whose cmdline is a Windows-style path
/// such as `C:\windows\system32\svchost.exe`. These are ignored by matching.
pub fn is_wine_app(raw_cmd: &str) -> bool {
    if raw_cmd.contains('\\') {
        return true;
    }
    // Windows drive prefix like `C:\` or `C:/`.
    let bytes = raw_cmd.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
}

/// Normalize an app name for tolerant matching between installed-app display
/// names (e.g. "Disk Usage Analyzer"), desktop exec binaries (e.g. "baobab"),
/// and running process names (e.g. "ardour9", version-suffixed AppImages).
///
/// Lowercases, strips known suffixes and version/hash tails, and removes all
/// non-alphanumeric characters (dashes, underscores, spaces, dots).
pub fn normalize(s: &str) -> String {
    let mut s = s.trim().to_lowercase();

    // Strip known file-type suffixes.
    for suffix in [".desktop", ".appimage", ".exe"] {
        if s.ends_with(suffix) {
            s.truncate(s.len() - suffix.len());
        }
    }

    // Drop a trailing version/hash tail introduced by a separator, e.g.
    // "deltachat-2.43.0-x86_64_0858..." -> "deltachat",
    // "capacities-1.63.3_3ec7..." -> "capacities".
    if let Some(idx) = s.find(|c| c == '-' || c == '_') {
        let tail = &s[idx + 1..];
        let tail_has_digit = tail.chars().any(|c| c.is_ascii_digit());
        if tail_has_digit {
            s.truncate(idx);
        }
    }

    // Remove everything that is not alphanumeric.
    s.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
}

/// Decide whether a running process binary matches any blocked app.
///
/// `blocked_norm` holds already-normalized blocked names (from the stored CSV,
/// which uses installed-app display names). `installed` maps exec-bin -> display
/// name so we can bridge the process-name / display-name namespaces.
pub fn matches_blocked(
    open_bin: &str,
    blocked_norm: &std::collections::HashSet<String>,
    installed: &HashMap<String, String>,
) -> bool {
    let bin_norm = normalize(open_bin);
    if !bin_norm.is_empty() && blocked_norm.contains(&bin_norm) {
        return true;
    }
    if let Some(display_name) = installed.get(open_bin) {
        let name_norm = normalize(display_name);
        if !name_norm.is_empty() && blocked_norm.contains(&name_norm) {
            return true;
        }
    }
    false
}
