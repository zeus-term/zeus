pub mod core;
pub mod platform;
pub mod utils;

use ::core::panic;
use core::{main_loop::start_main_loop, socket::connect_master};
use std::os::fd::AsFd;

use nix::unistd::pipe;

fn main() {
	let pipe_fds = pipe();
	if pipe_fds.is_err() {
		panic!("Error occured when creating a unix pipe");
	}
	let stream = connect_master();
	if let Ok(res) = start_main_loop(stream.as_fd()) {
		// terminated successfully
	}
}
