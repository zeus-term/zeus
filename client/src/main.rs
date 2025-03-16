pub mod core;
pub mod platform;
pub mod utils;

use ::core::panic;
use core::{main_loop::start_main_loop, socket::connect_master};

use nix::unistd::pipe;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
	let pipe_fds = pipe();
	if pipe_fds.is_err() {
		panic!("Error occured when creating a unix pipe");
	}
	let stream = connect_master();
	if let Ok(_res) = start_main_loop(stream) {}
}
