use crate::parsing::parse_event;

#[test]
fn ignore_v1_events() {
    assert_eq!(parse_event("workspace>>data".to_string()), None);
    assert_eq!(parse_event("focusedmon>>data".to_string()), None);
    assert_eq!(parse_event("activewindow>>data".to_string()), None);
    assert_eq!(parse_event("monitorremoved>>data".to_string()), None);
    assert_eq!(parse_event("monitoradded>>data".to_string()), None);
    assert_eq!(parse_event("createworkspace>>data".to_string()), None);
    assert_eq!(parse_event("destroyworkspace>>data".to_string()), None);
    assert_eq!(parse_event("moveworkspace>>data".to_string()), None);
    assert_eq!(parse_event("activespecial>>data".to_string()), None);
    assert_eq!(parse_event("movewindow>>data".to_string()), None);
    assert_eq!(parse_event("screencast>>data".to_string()), None);
    assert_eq!(parse_event("windowtitle>>data".to_string()), None);
}
