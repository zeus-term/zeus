use std::{
	io::{Read, Write},
	os::{
		fd::{AsRawFd, BorrowedFd, OwnedFd},
		unix::net::UnixStream,
	},
};

use common::{
	borrowed_fd,
	constants::{
		character::general_ascii_chars::NEWLINE, msg_directives::CREATE_PTY, socket::HERMES_COMM,
		STDOUT_FILENO,
	},
	forwarder::start_forwarder,
};
use nix::unistd::{dup, read, write};
use std::thread;

pub fn connect_master() -> UnixStream {
	let mut stream = UnixStream::connect(HERMES_COMM).unwrap();

	// request to create a new shell
	stream.write_all(CREATE_PTY).unwrap();
	stream.write_all(&[NEWLINE]).unwrap();

	let mut pts_path = String::new();

	let mut buffer: [u8; 1] = [0; 1];
	while stream.read_exact(&mut buffer).is_ok() {
		if buffer[0] == NEWLINE {
			break;
		}
		pts_path.push(buffer[0].into());
	}

	stream
}

pub fn start_socket_forwarding(socket: UnixStream, read_fd: OwnedFd) {
	let (socket_fd_in, socket_fd_out) = (socket.as_raw_fd(), dup(socket.as_raw_fd()).unwrap());

	let (in_task, out_task) = (
		thread::spawn(move || {
			start_forwarder(socket_fd_in, read_fd.as_raw_fd());
		}),
		thread::spawn(move || {
			start_forwarder(socket_fd_out, STDOUT_FILENO);
		}),
	);
	let _ = in_task.join();
	let _ = out_task.join();
}
