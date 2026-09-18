pub(crate) fn get_command_socket_path() -> Result<String, String> {
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
