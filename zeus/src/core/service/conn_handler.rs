use crate::core::constants::byte::{EOF, ESCAPE, NEWLINE};
use std::{io::Read, os::unix::net::UnixStream};

pub async fn handle_conn(mut socket: UnixStream) {
    let mut byte_buffer = vec![0u8; 1];
    let mut message_buffer: Vec<u8> = Vec::new();
    let mut should_escape: bool = false;

    while socket.read_exact(&mut byte_buffer).is_ok() {
        println!("Read: {}", byte_buffer[0]);
        if should_escape {
            should_escape = false;
            message_buffer.push(byte_buffer[0]);
            continue;
        }

        if byte_buffer[0] == ESCAPE {
            should_escape = true;
            continue;
        }

        if byte_buffer[0] == EOF {
            break;
        }

        if byte_buffer[0] == NEWLINE {
            println!(
                "Data string: {:?}",
                String::from_utf8(message_buffer.clone())
            );
            message_buffer.clear();
            continue;
        }

        message_buffer.push(byte_buffer[0])
    }
}
