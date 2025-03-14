use log::{error, info};
use postcard::{from_bytes, to_slice, Error};
use std::{
	io::{BufRead, BufReader, Write},
	os::{
		fd::{AsRawFd, FromRawFd},
		unix::net::{UnixListener, UnixStream},
	},
};

use common::{
	err::Error as ZError,
	protocol::{master::{HandleMessage, Message, Response}, utils::raw_response},
};

struct MessageHandler {
	buf_sock_stream: BufReader<UnixStream>,
	sock_fd: i32,
	sock_stream: UnixStream,
}

pub struct Context {
	pub(super) sock_fd: i32,
}

impl Context {
	pub fn new(sock_fd: i32) -> Context {
		Context { sock_fd }
	}
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
}

impl HandleMessage for MessageHandler {
	fn handle(&mut self, msg: Message) -> Response {
		let ctx = Context::new(self.sock_fd);

		match msg {
			Message::Init => super::command::init::handle(msg, ctx),
			_ => Response::Ack(-1),
		}
	}
}

impl MessageHandler {
	pub fn new(sock: UnixStream) -> MessageHandler {
		let sock_fd = sock.as_raw_fd();
		let sock_stream = unsafe { UnixStream::from_raw_fd(sock_fd) };

		MessageHandler {
			buf_sock_stream: BufReader::new(sock),
			sock_fd,
			sock_stream,
		}
	}

	pub async fn start_handler(&mut self) {
		loop {
			let response = match self.read_message() {
				Ok(msg) => self.handle(msg),
				Err(err) => {
					error!(
						"Error occurred while reading the message: {}, trace: {}",
						ZError::MessageParsingError,
						err
					);
					Response::Ack(-1)
				}
			};

			match raw_response(response) {
				Ok(raw_res) => {
					self.sock_stream.write(&raw_res);
				},
				Err(err) => {
					error!("{}", err);
				}
			}
		}
	}
}

pub fn handler(listener: UnixListener) {
	loop {
		info!("Starting v2 master listener...");
		match listener.accept() {
			Ok((stream, _)) => {
				let mut handler = MessageHandler::new(stream);
				tokio::spawn(async move {
					handler.start_handler().await;
				});
			}
			Err(err) => {
				error!(
					"Error setting up handler for established connection: {}",
					err
				);
			}
		}
	}
}
