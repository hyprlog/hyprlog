pub enum Event {
    Workspace {
        workspace_id: i64,
        workspace_name: String,
    },
    FocusedMon {
        monitor_name: String,
        workspace_id: i64,
    },
    ActiveWindow {
        window_address: Option<WindowAddress>,
    },
    Fullscreen {
        is_fullscreen: bool,
    },
    MonitorRemoved {
        monitor_id: i64,
        monitor_name: String,
        monitor_description: String,
    },
    MonitorAdded {
        monitor_id: i64,
        monitor_name: String,
        monitor_description: String,
    },
    CreateWorkspace {
        workspace_id: i64,
        workspace_name: String,
    },
    DestroyWorkspace {
        workspace_id: i64,
        workspace_name: String,
    },
    MoveWorkspace {
        workspace_id: i64,
        workspace_name: String,
        monitor_name: String,
    },
    RenameWorkspace {
        workspace_id: i64,
        new_name: String,
    },
    ActiveSpecial {
        workspace_id: Option<i64>,
        workspace_name: Option<String>,
        monitor_name: String,
    },
    ActiveLayout {
        keyboard_name: String,
        layout_name: String,
    },
    OpenWindow {
        window_address: WindowAddress,
        workspace_name: String,
        window_class: String,
        window_title: String,
    },
    CloseWindow {
        window_address: WindowAddress,
    },
    Kill {
        window_address: WindowAddress,
    },
    MoveWindow {
        window_address: String,
        workspace_id: i64,
        workspace_name: String,
    },
    OpenLayer {
        namespace: String,
    },
    CloseLayer {
        namespace: String,
    },
    Submap {
        submap_name: Option<String>,
    },
    ChangeFloatingMode {
        window_address: WindowAddress,
        is_floating: bool,
    },
    Urgent {
        window_address: String,
    },
    Screencast {
        state: bool,
        owner: String,
        name: String,
    },
    WindowTitle {
        window_address: WindowAddress,
        window_title: String,
    },
    ToggleGroup {
        state: bool,
        window_addresses: Vec<WindowAddress>,
    },
    MoveIntoGroup {
        window_address: WindowAddress,
    },
    MoveOutOfGroup {
        window_address: WindowAddress,
    },
    IgnoreGroupLock {
        enabled: bool,
    },
    LockGroups {
        enabled: bool,
    },
    ConfigReloaded,
    Pin {
        window_address: WindowAddress,
        is_pinned: bool,
    },
    Minimized {
        window_address: WindowAddress,
        is_minimized: bool,
    },
    Bell {
        window_address: Option<WindowAddress>,
    },
}

struct WindowAddress {
    address: String,
}

struct Window {
    address: WindowAddress,
    class: String,
    title: String,
}
