pub mod core;
pub mod platform;
pub mod utils;

use core::main_loop::start_main_loop;
use core::socket::{connect_master, start_socket_forwarding};
use std::io::{self};

use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() -> io::Result<()> {
	let (send, recv) = mpsc::channel::<Vec<u8>>(1);
	let stream = connect_master();
	let io_task = task::spawn(async move { start_main_loop(send).await });
	start_socket_forwarding(stream, recv).await;

	// TODO: handle error
	let _ = io_task.await;
	Ok(())
}
