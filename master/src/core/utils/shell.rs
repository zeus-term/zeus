use std::ffi::CStr;

use common::constants::unix_paths::ETC_PASSWD;
use nix::libc::getpwuid;
use nix::unistd::{close, read};
use nix::{
	fcntl::{open, OFlag},
	sys::stat::Mode,
};

pub fn get_user_shell(user_id: Option<u32>) -> String {
	let uid = user_id.unwrap_or(0);

	let fd = open(ETC_PASSWD, OFlag::O_RDONLY, Mode::S_IROTH).unwrap();
	let mut bytes: Vec<u8> = Vec::new();

	let mut buffer: [u8; 1024] = [0; 1024];

	while let Ok(bytes_read) = read(fd, &mut buffer) {
		if bytes_read == 0 {
			break;
		}
		bytes.extend(buffer.iter().take(bytes_read));
	}

	let shell_path = unsafe {
		let passwd_entry = getpwuid(uid);
		CStr::from_ptr((*passwd_entry).pw_shell)
			.to_str()
			.unwrap()
			.to_owned()
	};

	close(fd).unwrap();

	shell_path
}
