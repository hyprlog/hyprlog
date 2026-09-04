use std::path::Path;

pub(crate) fn get_event_socket_path() -> Result<String, String> {
    // resolve required env vars
    let xdg_runtime_dir = match std::env::var("XDG_RUNTIME_DIR") {
        Ok(val) => val,
        Err(_) => return Err("XDG_RUNTIME_DIR is not set".to_string()),
    };

    let hyprland_instance_signature = match std::env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(signature) if instance_is_active(&xdg_runtime_dir, &signature) => signature,
        _ => search_for_current_hyprland_instance_signature(
            &xdg_runtime_dir,
            std::env::var("WAYLAND_DISPLAY").ok().as_deref(),
        ),
    };

    // construct path
    let event_socket_path = format!(
        "{}/hypr/{}/.socket2.sock",
        xdg_runtime_dir, hyprland_instance_signature
    );
    return Ok(event_socket_path);
}

// fallback/verification code for when environment variables are not set properly
// this happened to me when testing from a stale tmux server
// warning: written by gpt-5.6 sol

fn instance_is_active(runtime_dir: &str, signature: &str) -> bool {
    let lock = Path::new(runtime_dir)
        .join("hypr")
        .join(signature)
        .join("hyprland.lock");
    let Ok(contents) = std::fs::read_to_string(lock) else {
        return false;
    };
    let Some(pid) = contents.lines().next() else {
        return false;
    };

    pid.parse::<u32>().is_ok() && Path::new("/proc").join(pid).exists()
}
fn search_for_current_hyprland_instance_signature(
    xdg_runtime_dir: &String,
    wayland_display: Option<&str>,
) -> String {
    let hypr_runtime_dir = Path::new(xdg_runtime_dir).join("hypr");
    let directory_entries = match std::fs::read_dir(&hypr_runtime_dir) {
        Ok(entries) => entries,
        Err(err) => panic!(
            "failed to read Hyprland runtime directory {}: {}",
            hypr_runtime_dir.display(),
            err
        ),
    };

    let mut instances = Vec::new();

    for entry_result in directory_entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(err) => panic!(
                "failed to read an entry in {}: {}",
                hypr_runtime_dir.display(),
                err
            ),
        };
        let instance_dir = entry.path();

        if !instance_dir.is_dir() {
            continue;
        }

        let event_socket_path = instance_dir.join(".socket2.sock");
        if !event_socket_path.exists() {
            continue;
        }

        let lock_contents = match std::fs::read_to_string(instance_dir.join("hyprland.lock")) {
            Ok(contents) => contents,
            Err(_) => continue,
        };

        let mut lock_lines = lock_contents.lines();
        let pid = match lock_lines.next() {
            Some(pid) => pid,
            None => continue,
        };
        let instance_wayland_display = match lock_lines.next() {
            Some(display) => display,
            None => continue,
        };

        if pid.parse::<u32>().is_err() {
            continue;
        }

        let process_path = Path::new("/proc").join(pid);
        if !process_path.exists() {
            continue;
        }

        let signature = match instance_dir.file_name().and_then(|name| name.to_str()) {
            Some(signature) => signature.to_string(),
            None => continue,
        };

        instances.push((signature, instance_wayland_display.to_string()));
    }

    if let Some(current_wayland_display) = wayland_display {
        for (signature, instance_wayland_display) in &instances {
            if instance_wayland_display == current_wayland_display {
                return signature.clone();
            }
        }
    }

    if instances.len() == 1 {
        return instances[0].0.clone();
    }

    if instances.is_empty() {
        panic!("no running Hyprland instances were found");
    }

    panic!(
        "found {} running Hyprland instances, but could not determine which one is current",
        instances.len()
    )
}
