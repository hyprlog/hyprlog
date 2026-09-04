use crate::events::Event;
pub fn parse_event(event: String) -> Option<Event> {
    println!("{}", event);
    if let Some((event_name, event_data)) = event.split_once(">>") {
        println!("{}", event_name);
        println!("{}", event_data);
        // match event_name {
        //     "workspacev2" => {}
        //     "focusedmonv2" => {}
        //     "activewindowv2" => {}
        //     "fullscreen" => {}
        //     "monitorremovedv2" => {}
        //     "monitoraddedv2" => {}
        //     "createworkspacev2" => {}
        //     "destroyworkspacev2" => {}
        //     "moveworkspacev2" => {}
        //     "renameworkspace" => {}
        //     "activespecialv2" => {}
        //     "activelayout" => {}
        //     "openwindow" => {}
        //     "closewindow" => {}
        //     "kill" => {}
        //     "movewindowv2" => {}
        //     "openlayer" => {}
        //     "closelayer" => {}
        //     "submap" => {}
        //     "changefloatingmode" => {}
        //     "urgent" => {}
        //     "screencastv2" => {}
        //     "windowtitlev2" => {}
        //     "togglegroup" => {}
        //     "moveintogroup" => {}
        //     "moveoutofgroup" => {}
        //     "ignoregrouplock" => {}
        //     "lockgroups" => {}
        //     "configreloaded" => {}
        //     "pin" => {}
        //     "minimized" => {}
        //     "bell" => {}
        //     _ => return None,
        // }
    }
    return None;
}
