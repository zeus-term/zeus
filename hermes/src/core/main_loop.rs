use std::{
	io::{self, Write},
	os::fd::{AsFd, AsRawFd, BorrowedFd},
};

use common::borrowed_fd;
use nix::unistd::{dup, read, write};
use std::thread;

use crate::utils::buffer::handle_input;

use super::init::get_term_state;

pub fn start_main_loop(fd: BorrowedFd) -> io::Result<()> {
	let (mut handler, mut buffer, key_mapper) = get_term_state();
	let sync_fd = dup(fd.as_raw_fd());
	let join_handle = thread::spawn(move || {
		let mut read_buf: [u8; 10] = [0; 10];
		loop {
			if let Ok(read_bytes) = read(sync_fd.unwrap(), &mut read_buf) {
				if read_bytes > 0 {
					let _ = write(borrowed_fd!(0), &read_buf);
				}
			}
		}
	});

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
				let _ = write(fd.as_fd(), &data);
			}
		} else {
			buffer.flush_buffer();
		}
		io::stdout().flush()?;
	}
	let _ = join_handle.join();
	Ok(())
}
