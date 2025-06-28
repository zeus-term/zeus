use log::{debug, error};
use nix::pty::PtyMaster;
use postcard::{from_bytes, Error};
use std::{
	io::BufRead,
	io::{BufReader, Write},
	os::{
		fd::{AsRawFd, FromRawFd},
		linux,
		unix::net::UnixStream,
	},
	sync::Arc,
};
#[cfg(feature = "client")]
use tokio::sync::oneshot::Sender;

use crate::err::Error as ZError;

use super::{message::Message, utils::raw_message};

#[cfg(feature = "master")]
pub struct Context {
	pub sock_fd: i32,
	pub master: Option<PtyMaster>,
}

#[cfg(feature = "master")]
impl Context {
	pub fn new(sock_fd: i32) -> Context {
		Context {
			sock_fd,
			master: None,
		}
	}
}

#[cfg(feature = "client")]
pub struct Context {
	pub sock_fd: i32,
	pub pid_tx: Sender<i32>,
}
#[cfg(feature = "client")]
impl Context {
	pub fn new(sock_fd: i32, pid_tx: Sender<i32>) -> Context {
		Context { sock_fd, pid_tx }
	}
}

type Handler = fn(Message, &Context) -> Option<Message>;

pub struct MessageHandler {
	pub buf_sock_stream: BufReader<UnixStream>,
	pub sock_fd: i32,
	pub sock_stream: UnixStream,
	pub handle: Handler,
	pub ctx: Context,
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

	#[cfg(feature = "master")]
	pub fn new(sock: UnixStream, handle: Handler) -> MessageHandler {
		let sock_fd = sock.as_raw_fd();
		let sock_stream = unsafe { UnixStream::from_raw_fd(sock_fd) };

		MessageHandler {
			buf_sock_stream: BufReader::new(sock),
			sock_fd,
			sock_stream,
			handle,
			ctx: Context::new(sock_fd),
		}
	}

	#[cfg(feature = "client")]
	pub fn new(sock: UnixStream, handle: Handler, pid_tx: Sender<i32>) -> MessageHandler {
		let sock_fd = sock.as_raw_fd();
		let sock_stream = unsafe { UnixStream::from_raw_fd(sock_fd) };

		MessageHandler {
			buf_sock_stream: BufReader::new(sock),
			sock_fd,
			sock_stream,
			handle,
			ctx: Context::new(sock_fd, pid_tx),
		}
	}

	pub async fn start_handler(&mut self) {
		loop {
			let response = match self.read_message() {
				Ok(msg) => match (self.handle)(msg, &self.ctx) {
					Some(res) => res,
					None => {
						continue;
					}
				},
				Err(err) => {
					error!(
						"Error occurred while reading the message: {}, trace: {}",
						ZError::MessageParsingError,
						err
					);
					Message::Ack(-1)
				}
			};
			debug!("Message : {:?}", response);

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
