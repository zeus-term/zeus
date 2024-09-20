use std::io::{self, Read, Stderr, Stdin, Stdout, Write};

use nix::libc;

pub struct IOHandler {
	// Don't have to tell what these are :)
	in_stream: Stdin,
	out_stream: Stdout,
	err_stream: Stderr,
}

impl IOHandler {
	pub fn new() -> IOHandler {
		IOHandler {
			in_stream: io::stdin(),
			out_stream: io::stdout(),
			err_stream: io::stderr(),
		}
	}

	pub fn read(&mut self) -> std::io::Result<u8> {
		let mut data = vec![0u8; 1];
		match self.in_stream.read(&mut data) {
			Ok(_) => Ok(data[0]),
			Err(err) => Err(err),
		}
	}

	pub fn write_byte(&mut self, data: u8) -> std::io::Result<()> {
		match self.out_stream.write(&[data]) {
			Ok(_) => Ok(()),
			Err(err) => Err(err),
		}
	}

	pub fn write_str(&mut self, data: &[u8]) -> std::io::Result<()> {
		match self.out_stream.write(data) {
			Ok(_) => Ok(()),
			Err(err) => Err(err),
		}
	}

	pub fn disable_line_buffering(&mut self) -> io::Result<()> {
		let mut termios = core::mem::MaybeUninit::uninit();
		unsafe {
			libc::tcgetattr(0, termios.as_mut_ptr());
		}

		let mut termios = unsafe { termios.assume_init() };

		termios.c_iflag &= !(libc::IGNBRK | libc::BRKINT | libc::IXON | libc::PARMRK | libc::ICRNL);
		termios.c_lflag &= !(libc::ISIG | libc::ECHO | libc::ECHONL | libc::ICANON);
		termios.c_oflag |= libc::OPOST;

		unsafe {
			libc::tcsetattr(0, libc::TCSANOW, &termios);
		}

		Ok(())
	}
}

impl Default for IOHandler {
	fn default() -> Self {
		Self::new()
	}
}
