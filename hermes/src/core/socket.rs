use std::{
	io::{Read, Write},
	os::{
		fd::{AsRawFd, BorrowedFd},
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
use nix::unistd::{dup, write};
use tokio::sync::mpsc::Receiver;

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

pub async fn start_socket_forwarding(socket: UnixStream, recv: Receiver<Vec<u8>>) {
	let (socket_fd_in, socket_fd_out) = (socket.as_raw_fd(), dup(socket.as_raw_fd()).unwrap());

	let (in_task, out_task) = (
		tokio::task::spawn(async move {
			start_socket_write_forwarding(socket_fd_in, recv).await;
		}),
		tokio::task::spawn_blocking(move || {
			start_forwarder(socket_fd_out, STDOUT_FILENO);
		}),
	);
	in_task.await;
	out_task.await;
}

async fn start_socket_write_forwarding(socket_fd: i32, mut recv: Receiver<Vec<u8>>) {
	loop {
		match recv.recv().await {
			Some(data) => {
				write(borrowed_fd!(socket_fd), &data);
			}
			None => {
				continue;
			}
		}
	}
}

async fn start_socket_read_forwarding(socket_fd: i32) {
	start_forwarder(socket_fd, STDOUT_FILENO);
}
