pub mod core;
pub mod platform;
pub mod utils;

use ::core::panic;
use core::{
	main_loop::start_main_loop,
	socket::{connect_master, start_socket_forwarding},
};
use std::thread;

use nix::unistd::pipe;

fn main() {
	let pipe_fds = pipe();
	if pipe_fds.is_err() {
		panic!("Error occured when creating a unix pipe");
	}
	let (read_fd, write_fd) = pipe_fds.unwrap();
	let stream = connect_master();
	let io_task = thread::spawn(|| start_main_loop(write_fd));
	start_socket_forwarding(stream, read_fd);
	let err = io_task.join();
}
