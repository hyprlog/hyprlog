mod events;
mod parsing;
mod socket2;

#[cfg(test)]
#[path = "tests/parsing.rs"]
mod parsing_tests;

use socket2::get_event_socket_path;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

use crate::parsing::parse_event;

fn main() -> std::io::Result<()> {
    // get path to hyprland event socket
    let event_socket_path = match get_event_socket_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // connect to hyprland event socket, start listening for events
    match UnixStream::connect(event_socket_path) {
        Ok(mut stream) => {
            /* connection succeeded */
            handle_event_stream(stream);
        }
        Err(err) => {
            /* connection failed */
            eprintln!("failed to connect to event socket: {}", err);
        }
    }

    Ok(())
}

fn handle_event_stream(mut event_stream: UnixStream) {
    let reader = BufReader::new(&event_stream);
    for line in reader.lines() {
        if let Some(line) = match line {
            Ok(line) => Some(line),
            Err(e) => {
                eprintln!("failed to read line from event stream: {}", e);
                None
            }
        } {
            let event = parse_event(line);
        }
    }
}
