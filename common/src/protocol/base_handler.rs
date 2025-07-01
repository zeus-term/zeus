use std::{
	io::{BufReader, Read, Write},
	os::unix::net::UnixStream,
};

use log::debug;
use postcard::{from_bytes, to_slice};

use super::message::Message;

pub trait MessageHandler<T> {
	fn handle(msg: Message, ctx: &T) -> Option<Message>;

	fn get_context(&mut self) -> &T;

	fn get_read_stream(&mut self) -> BufReader<UnixStream>;

	fn get_write_stream(&mut self) -> &mut UnixStream;

	fn start_handler(&mut self) {
		let mut reader = self.get_read_stream();
		debug!("Started v2 handler");

		loop {
			let mut buf: [u8; 1] = [0; 1];
			let mut size = 0;
			if reader.read_exact(&mut buf).is_ok() {
				if buf[0] <= 0xFC {
					size = buf[0] as usize;
				} else if buf[0] == 0xFD {
					let mut buf: [u8; 2] = [0; 2];
					let _ = reader.read_exact(&mut buf);
					size = u16::from_le_bytes(buf) as usize;
				} else if buf[0] == 0xFE {
					let mut buf: [u8; 4] = [0; 4];
					let _ = reader.read_exact(&mut buf);
					size = u32::from_le_bytes(buf) as usize;
				} else if buf[0] == 0xFF {
					let mut buf: [u8; 8] = [0; 8];
					let _ = reader.read_exact(&mut buf);
					size = u64::from_le_bytes(buf) as usize;
				}

				// Read the actual data based on the size we determined
				let mut data_buf = vec![0u8; size];
				if reader.read_exact(&mut data_buf).is_ok() {
					// Process the data here
					match from_bytes::<Message>(&data_buf) {
						Ok(msg) => {
							if let Some(response) = Self::handle(msg, self.get_context()) {
								if let Err(e) = self.send_response(&response) {
									eprintln!("Error sending response: {}", e);
								}
							}
						}
						Err(_err) => {}
					}
				} else {
					// Handle read error
					break;
				}
			} else {
				// Handle initial read error
				break;
			}
		}
	}

	fn send_response(&mut self, response: &Message) -> std::io::Result<()> {
		let mut buf = [0u8; 32];
		let encoded = to_slice(response, &mut buf)
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
		let x = encoded.to_vec();

		let mut wire_size = get_wire_size(encoded.len());
		let raw_msg = [&mut wire_size, encoded].concat();
		let _ = self.get_write_stream().write_all(&raw_msg);
		self.get_write_stream().flush()
	}
}

fn get_wire_size(size: usize) -> Vec<u8> {
	if size <= 0xFC {
		vec![size as u8]
	} else if size <= 0xFFFF {
		vec![
			0xFD,
			(size & 0xFF) as u8,        // LSB first (little-endian)
			((size >> 8) & 0xFF) as u8, // Next byte
		]
	} else if size <= 0xFFFFFFFF {
		vec![
			0xFE,
			(size & 0xFF) as u8, // LSB first
			((size >> 8) & 0xFF) as u8,
			((size >> 16) & 0xFF) as u8,
			((size >> 24) & 0xFF) as u8,
		]
	} else {
		vec![
			0xFF,
			(size & 0xFF) as u8, // LSB first
			((size >> 8) & 0xFF) as u8,
			((size >> 16) & 0xFF) as u8,
			((size >> 24) & 0xFF) as u8,
			((size >> 32) & 0xFF) as u8,
			((size >> 40) & 0xFF) as u8,
			((size >> 48) & 0xFF) as u8,
			((size >> 56) & 0xFF) as u8,
		]
	}
}
