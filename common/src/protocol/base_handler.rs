use std::{
	io::BufRead,
	io::{BufReader, Write},
	os::{
		fd::{AsRawFd, FromRawFd},
		unix::net::UnixStream,
	},
};

use log::error;
use postcard::{from_bytes, Error};

use crate::err::Error as ZError;

use super::{master::Message, utils::raw_message};

#[cfg(feature = "master")]
pub struct Context {
	pub sock_fd: i32,
}

#[cfg(feature = "master")]
impl Context {
	pub fn new(sock_fd: i32) -> Context {
		Context { sock_fd }
	}
}

#[cfg(feature = "client")]
pub struct Context {
	pub sock_fd: i32,
}
#[cfg(feature = "client")]
impl Context {
	pub fn new(sock_fd: i32) -> Context {
		Context { sock_fd }
	}
}

type Handler = fn(Message, Context) -> Message;

pub struct MessageHandler {
	pub buf_sock_stream: BufReader<UnixStream>,
	pub sock_fd: i32,
	pub sock_stream: UnixStream,
	pub handle: Handler,
}

impl MessageHandler {
	pub fn read_message(&mut self) -> Result<Message, ZError> {
		let mut buf = String::new();
		if let Ok(res) = self.buf_sock_stream.read_line(&mut buf) {
			let msg: Result<Message, Error> = from_bytes(buf.as_bytes());

			return Ok(msg.unwrap());
		}

		Err(ZError::SocketReadError)
	}

	pub fn new(sock: UnixStream, handle: Handler) -> MessageHandler {
		let sock_fd = sock.as_raw_fd();
		let sock_stream = unsafe { UnixStream::from_raw_fd(sock_fd) };

		MessageHandler {
			buf_sock_stream: BufReader::new(sock),
			sock_fd,
			sock_stream,
			handle,
		}
	}

	#[cfg(feature = "master")]
	pub fn prepare_context(&self) -> Context {
		Context {
			sock_fd: self.sock_fd,
		}
	}

	#[cfg(feature = "client")]
	pub fn prepare_context(&self) -> Context {
		Context {
			sock_fd: self.sock_fd,
		}
	}

	pub async fn start_handler(&mut self) {
		loop {
			let ctx = self.prepare_context();
			let response = match self.read_message() {
				Ok(msg) => (self.handle)(msg, ctx),
				Err(err) => {
					error!(
						"Error occurred while reading the message: {}, trace: {}",
						ZError::MessageParsingError,
						err
					);
					Message::Ack(-1)
				}
			};

			match raw_message(response) {
				Ok(raw_res) => {
					self.sock_stream.write(&raw_res);
				}
				Err(err) => {
					error!("{}", err);
				}
			}
		}
	}
}
