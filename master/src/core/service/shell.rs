use std::ffi::{CStr, CString};

use common::{
	constants::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO},
	strings::get_cstr,
};
use nix::{
	fcntl::{open, OFlag},
	sys::stat::Mode,
	unistd::{close, dup2, execve, getuid},
};

use crate::core::utils::shell::get_user_shell;

pub fn start_shell_subprocess(ptsname: &str) {
	let pts_fd = open(ptsname, OFlag::O_RDWR, Mode::S_IRWXU).unwrap();
	dup2(pts_fd, STDIN_FILENO).unwrap();
	dup2(pts_fd, STDOUT_FILENO).unwrap();
	dup2(pts_fd, STDERR_FILENO).unwrap();

	// Clean up unused file descriptors
	close(pts_fd).unwrap();

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
