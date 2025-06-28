use std::{
	io::{BufReader, Read, Write},
	os::unix::net::UnixStream,
};

use postcard::{from_bytes, to_slice};

use super::message::Message;
use crate::constants::character::general_ascii_chars::EOF;

pub trait MessageHandler<T> {
	fn handle(msg: Message, ctx: &T) -> Option<Message>;

	fn get_context(&mut self) -> &T;

	fn get_read_stream(&mut self) -> BufReader<UnixStream>;

	fn get_write_stream(&mut self) -> &mut UnixStream;

	fn start_handler(&mut self) {
		let mut reader = self.get_read_stream();
		let mut buf = Vec::new();

		loop {
			// Read until EOF (stream closes)
			match reader.read_to_end(&mut buf) {
				Ok(_) => {
					// Remove the EOF byte if present
					if let Some(&last) = buf.last() {
						if last == EOF {
							buf.pop();
						}
					}

					match from_bytes::<Message>(&buf) {
						Ok(msg) => {
							if let Some(response) = Self::handle(msg, self.get_context()) {
								if let Err(e) = self.send_response(&response) {
									eprintln!("Error sending response: {}", e);
								}
							}
						}
						Err(e) => eprintln!("Failed to deserialize message: {}", e),
					}
				}
				Err(e) => eprintln!("Failed to read from stream: {}", e),
			}
		}
	}

	fn send_response(&mut self, response: &Message) -> std::io::Result<()> {
		let mut buf = [0u8; 32];
		let encoded = to_slice(response, &mut buf)
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

		let raw_msg = [encoded, &mut [EOF]].concat();
		self.get_write_stream().write_all(&raw_msg)
	}
}
