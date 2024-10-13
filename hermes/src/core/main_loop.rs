use std::io::{self, Write};

use crate::utils::buffer::handle_input;

use super::init::get_term_state;
use tokio::sync::mpsc::Sender;

pub async fn start_main_loop(sender: Sender<Vec<u8>>) -> io::Result<()> {
	let (mut handler, mut buffer, key_mapper) = get_term_state();

	handler.disable_line_buffering()?;
	while let Ok(data) = handler.read() {
		let mut keys: Vec<u8> = Vec::new();

		if !buffer.in_buf.is_empty() {
			keys.extend_from_slice(&buffer.in_buf);
		}

		keys.push(data);

		if let Ok(callback) = key_mapper.key_fn(&keys) {
			let data = handle_input(callback(), &mut buffer, &mut handler, false);
			if let Some(data) = data {
				sender.send(data.clone()).await.unwrap();
			}
		} else {
			buffer.flush_buffer();
		}
		io::stdout().flush()?;
	}
	Ok(())
}
