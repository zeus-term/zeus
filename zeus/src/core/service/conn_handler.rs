use common::{
	constants::{
		character::general_ascii_chars::{EOF, ESC, NEWLINE},
		msg_directives::CREATE_PTY,
	},
	match_arr,
};
use log::info;
use nix::{
	fcntl::OFlag,
	pty::{grantpt, posix_openpt, ptsname_r, unlockpt, PtyMaster},
};
use std::{
	fmt::Display,
	io::{Read, Write},
	os::unix::net::UnixStream,
};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct PtyCreationError {}

impl Display for PtyCreationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Error creating pty")
	}
}

/// This creates the psuedo-terminal and returns the
fn create_pty() -> Result<(PtyMaster, String), PtyCreationError> {
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
				Err(_) => Err(PtyCreationError {}),
			}
		}
		Err(_) => Err(PtyCreationError {}),
	}
}

pub async fn handle_conn(mut socket: UnixStream, tx: mpsc::Sender<PtyMaster>) {
	let mut byte_buffer = vec![0u8; 1];
	let mut message_buffer: Vec<u8> = Vec::new();
	let mut should_escape: bool = false;

	while socket.read_exact(&mut byte_buffer).is_ok() {
		if should_escape {
			should_escape = false;
			message_buffer.push(byte_buffer[0]);
			continue;
		}

		if byte_buffer[0] == ESC {
			should_escape = true;
			continue;
		}

		if byte_buffer[0] == EOF {
			break;
		}

		if byte_buffer[0] == NEWLINE {
			match_arr!(message_buffer, {
				CREATE_PTY => {
					let mut pty_vec: Vec<PtyMaster> = Vec::new();
					if let Ok((master, ptyname)) = create_pty() {
						pty_vec.push(master);
						socket.write_all(ptyname.as_bytes()).unwrap();
					}
					if let Ok((master, ptyname)) = create_pty() {
						pty_vec.push(master);
						socket.write_all(ptyname.as_bytes()).unwrap();
					}

					for pty in pty_vec {
						tx.send(pty).await.unwrap();
					}
				},
			});
			break;
		}

		message_buffer.push(byte_buffer[0])
	}
}
