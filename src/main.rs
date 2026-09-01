mod socket2;

use socket2::get_event_socket_path;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

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
    loop {
        let reader = BufReader::new(&event_stream);
        for line in reader.lines() {
            println!("event: {:?}", line);
        }
    }
}
