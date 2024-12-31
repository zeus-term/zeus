use std::os::fd::{AsFd, BorrowedFd};

use log::debug;
use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
use nix::unistd::{read, write};

use crate::borrowed_fd;

pub fn start_forwarder(from_fd: i32, to_fd: i32) {
	debug!(
		"Request to start forward from : {}, to : {}",
		from_fd, to_fd
	);
	let mut buf: [u8; 512] = [0; 512];

	loop {
		let in_fd = PollFd::new(borrowed_fd!(from_fd), PollFlags::POLLIN);

		let mut rd_fds: [PollFd; 1] = [in_fd];

		// block till some data is available for read
		let result = poll(&mut rd_fds, PollTimeout::MAX);

		if result.is_err() || result.unwrap() <= 0 {
			continue;
		}

		let bytes_read = read(from_fd, &mut buf).unwrap_or(0);
		if bytes_read > 0 {
			let _ = write(borrowed_fd!(to_fd).as_fd(), &buf[0..bytes_read]);
		}
	}
}
