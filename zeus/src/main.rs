pub mod core;

use crate::core::service::conn_handler::handle_conn;
use common::constants::socket::HERMES_COMM;
use log::{error, info};
use nix::{
	pty::PtyMaster,
	sys::wait::waitpid,
	unistd::{fork, Pid},
};
use simple_logger::SimpleLogger;
use std::{
	io::{self},
	os::unix::net::UnixListener,
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
	SimpleLogger::new().env().init().unwrap();

	info!("Bootstrap process started");

	let _ = std::fs::remove_file(HERMES_COMM);
	info!("Cleaning up unix sockets");

	let listener = UnixListener::bind(HERMES_COMM)?;
	let mut completion_engine_ptymaster: Option<PtyMaster> = None;
	let mut shell_pty_master: Option<PtyMaster> = None;

	let mut children: Vec<Pid> = Vec::new();

	loop {
		info!("Started to listen on sock");
		match listener.accept() {
			Ok((socket, addr)) => {
				info!("New connection from {:?}", addr);

				let (send, mut recv) = mpsc::channel::<PtyMaster>(2);

				tokio::task::spawn(async move {
					handle_conn(socket, send).await;
				});

				let mut pty_buffer: Vec<PtyMaster> = Vec::new();
				recv.recv_many(&mut pty_buffer, 2).await;
				match unsafe { fork() } {
					Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
						children.push(child);
					}
					Ok(nix::unistd::ForkResult::Child) => {
						completion_engine_ptymaster = Some(pty_buffer.remove(0));
						shell_pty_master = Some(pty_buffer.remove(0));
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

	if let (Some(engine_pty_master), Some(shell_pty_master)) =
		(completion_engine_ptymaster, shell_pty_master)
	{
	} else {
		info!("Waiting all the children...");
		for child in children.iter() {
			waitpid(*child, None).unwrap();
		}
	}

	Ok(())
}
