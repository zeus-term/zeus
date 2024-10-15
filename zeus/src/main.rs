pub mod core;

use common::constants::socket::HERMES_COMM;
use core::service::request_handler::serve_request;
use core::service::shell::fork_shell;
use core::utils::socket::cleanup_socket;
use log::{error, info};
use nix::unistd::{fork, ForkResult, Pid};
use simple_logger::SimpleLogger;
use std::{
	io::{self},
	os::unix::net::UnixListener,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> io::Result<()> {
	SimpleLogger::new().env().init().unwrap();

	info!("Bootstrap process started");

	cleanup_socket(HERMES_COMM);

	let listener = UnixListener::bind(HERMES_COMM)?;

	let mut children: Vec<Pid> = Vec::new();

	loop {
		info!("Listening for a connection to get accepted");
		match listener.accept() {
			Ok((socket, addr)) => {
				info!("New connection from {:?}", addr);

				let (recv_pty, recv_stream) = serve_request(socket).await;

				match unsafe { fork() } {
					Ok(ForkResult::Parent { child, .. }) => {
						children.push(child);
					}
					Ok(ForkResult::Child) => {
						info!("Forking shell...");
						let pty = recv_pty.await.unwrap();
						let stream = recv_stream.await.unwrap();
						fork_shell(pty, stream).await;
						break;
					}
					Err(err) => {
						error!("Error occured when creating a fork of zeus master: {}", err);
					}
				}
			}
			Err(err) => {
				error!("Failed to accept connection: {}", err);
			}
		}
	}

	Ok(())
}
