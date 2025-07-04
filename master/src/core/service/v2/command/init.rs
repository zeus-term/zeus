use std::os::fd::AsRawFd;

use common::{err::Error, forwarder::FdForward, protocol::message::Message};
use log::{debug, info};
use nix::{
	fcntl::OFlag,
	pty::{grantpt, posix_openpt, ptsname_r, unlockpt, PtyMaster},
};

use super::super::context::Context;
use super::z_fork::fork_process;

/// Message handler for INIT event
/// Refer https://github.com/zeus-term/zeus/
pub fn handle(_msg: Message, ctx: &Context) -> Message {
	info!("in init");
	if let Ok((pty_master, pty_path)) = create_pty() {
		let pty_to_sock = FdForward {
			from: ctx.sock_fd,
			to: pty_master.as_raw_fd(),
		};

		if let Ok(res) = fork_process(&[pty_to_sock], pty_path.as_str()) {
			let pid = match res {
				super::z_fork::ZForkResponse::Parent(pid) => Some(pid.as_raw()),
				_ => None,
			};
			return Message::AckPty(pty_path, pid);
		}

		debug!("Sending -1");
		return Message::Ack(-1);
	}
	debug!("Sending -1");
	Message::Ack(-1)
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
