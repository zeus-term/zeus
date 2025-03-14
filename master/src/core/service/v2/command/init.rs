use std::os::fd::AsRawFd;

use common::{
	err::Error,
	forwarder::FdForward,
	protocol::master::{Message, Response},
};
use log::info;
use nix::{
	fcntl::OFlag,
	pty::{grantpt, posix_openpt, ptsname_r, unlockpt, PtyMaster},
};

use crate::core::service::v2::handler::Context;

use super::z_fork::fork_process;

/// Message handler for INIT event
/// Refer https://github.com/zeus-term/zeus/
pub fn handle(msg: Message, ctx: Context) -> Response {
	if let Ok((pty_master, pty_path)) = create_pty() {
		let sock_to_pty = FdForward {
			to: pty_master.as_raw_fd(),
			from: ctx.sock_fd,
		};
		let pty_to_sock = FdForward {
			from: ctx.sock_fd,
			to: pty_master.as_raw_fd(),
		};
		let _ = fork_process(sock_to_pty, pty_to_sock);
		return Response::AckPty(pty_path);
	}
	Response::Ack(-1)
}

/// This creates the psuedo-terminal and returns the PtyMaster object and slave path
fn create_pty() -> Result<(PtyMaster, String), Error> {
	info!("Creating new pty");
	match posix_openpt(OFlag::O_RDWR) {
		Ok(master) => {
			unlockpt(&master).unwrap();
			grantpt(&master).unwrap();

			match ptsname_r(&master) {
				Ok(ptsname) => {
					info!("Created pty and unlocked, ptsname: {}", ptsname);
					Ok((master, ptsname))
				}
				Err(_) => Err(Error::PtyCreationError),
			}
		}
		Err(_) => Err(Error::PtyCreationError),
	}
}
