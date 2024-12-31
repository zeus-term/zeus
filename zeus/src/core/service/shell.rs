use std::{
	ffi::{CStr, CString},
	os::{fd::AsRawFd, unix::net::UnixStream},
};

use common::{
	constants::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO},
	forwarder::start_forwarder,
	strings::get_cstr,
};
use log::{debug, error, info};
use nix::{
	fcntl::{open, OFlag},
	pty::{ptsname_r, PtyMaster},
	sys::{stat::Mode, wait::waitpid},
	unistd::{close, dup, dup2, execve, fork, getuid, ForkResult},
};

use crate::core::utils::shell::get_user_shell;

fn start_shell_subprocess(ptymaster: &PtyMaster) {
	let ptsname = ptsname_r(ptymaster).unwrap();

	let pts_fd = open(ptsname.as_str(), OFlag::O_RDWR, Mode::S_IRWXU).unwrap();
	dup2(pts_fd, STDIN_FILENO).unwrap();
	dup2(pts_fd, STDOUT_FILENO).unwrap();
	dup2(pts_fd, STDERR_FILENO).unwrap();

	// Clean up unused file descriptors
	close(pts_fd).unwrap();
	close(ptymaster.as_raw_fd()).unwrap();

	let real_uid = getuid().as_raw();
	let mut shell_path = get_user_shell(Some(real_uid));
	let shell_path_cstr = unsafe { get_cstr(shell_path.as_mut_str()) };
	let term_var = CString::new("TERM=xterm-256color").unwrap();
	let term = term_var.as_c_str();
	let args: &[&CStr] = &[shell_path_cstr];
	let null_str = CString::new("").unwrap();
	let environ: &[&CStr] = &[term, null_str.as_c_str()];

	// Start the shell process
	execve(shell_path_cstr, args, environ).unwrap();
}

pub async fn fork_shell(ptymaster: PtyMaster, stream: UnixStream) {
	info!("Forking new shell");
	match unsafe { fork() } {
		Ok(ForkResult::Parent { child }) => {
			// should have the process to forward the data from hermes communicator
			let (stream_in_fd, stream_out_fd) =
				(stream.as_raw_fd(), dup(stream.as_raw_fd()).unwrap());
			let (master_in_fd, master_out_fd) =
				(ptymaster.as_raw_fd(), dup(ptymaster.as_raw_fd()).unwrap());
			info!("Starting data forwarding task...");

			let (task1, task2) = (
				tokio::task::spawn_blocking(move || {
					start_forwarder(stream_in_fd, master_out_fd);
				}),
				tokio::task::spawn_blocking(move || {
					start_forwarder(master_in_fd, stream_out_fd);
				}),
			);

			debug!(
				"Waiting for all forwarder task to terminate... {:?} {:?}",
				task1, task2
			);

			// TODO: Handle error
			let _ = task1.await;
			let _ = task2.await;

			debug!("Forwarder task terminated, shutting down...");

			// TODO: Handle error
			let _ = waitpid(child, None);
		}
		Ok(ForkResult::Child) => {
			start_shell_subprocess(&ptymaster);
		}
		Err(_) => {
			error!("Error forking process");
		}
	}
}
