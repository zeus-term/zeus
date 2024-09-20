use std::os::fd::AsRawFd;

use nix::{
	pty::PtyMaster,
	unistd::{dup2, execve, fork},
};

pub fn fork_shell(ptymaster: PtyMaster) {
	// Set standard io file descriptors to psuedo-terminals master end
	let _ = dup2(0, ptymaster.as_raw_fd());
	let _ = dup2(1, ptymaster.as_raw_fd());
	let _ = dup2(2, ptymaster.as_raw_fd());

	// execve(path, args, env);
}
