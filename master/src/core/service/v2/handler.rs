use common::protocol::{base_handler::MessageHandler, message::Message};
use log::{error, info};
use std::{
	io::BufReader,
	os::{
		fd::{AsRawFd, FromRawFd},
		unix::net::{UnixListener, UnixStream},
	},
};

use super::context::Context;

pub fn handle(msg: Message, ctx: &Context) -> Option<Message> {
	match msg {
		Message::Init => Some(super::command::init::handle(msg, ctx)),
		Message::Forward(size, data) => Some(super::command::forward::handle(size, data, ctx)),
		Message::Ack(_val) => None,
		_ => Some(Message::Ack(-1)),
	}
}

pub struct MasterMessageHandler {
	ctx: Context,
	stream: UnixStream,
}

impl MasterMessageHandler {
	pub fn new(ctx: Context, stream: UnixStream) -> MasterMessageHandler {
		MasterMessageHandler { ctx, stream }
	}
}

impl MessageHandler<Context> for MasterMessageHandler {
	fn handle(msg: Message, ctx: &Context) -> Option<Message> {
		match msg {
			Message::Init => Some(super::command::init::handle(msg, ctx)),
			Message::Forward(size, data) => Some(super::command::forward::handle(size, data, ctx)),
			Message::Ack(_val) => None,
			_ => Some(Message::Ack(-1)),
		}
	}

	fn get_context(&mut self) -> &Context {
		&self.ctx
	}

	fn get_read_stream(&mut self) -> BufReader<UnixStream> {
		let sock_fd = self.stream.as_raw_fd();
		let unix_stream: UnixStream = unsafe { UnixStream::from_raw_fd(sock_fd) };

		BufReader::new(unix_stream)
	}

	fn get_write_stream(&mut self) -> &mut UnixStream {
		&mut self.stream
	}
}

pub fn request_handler(listener: UnixListener) {
	loop {
		info!("Starting v2 master listener...");
		match listener.accept() {
			Ok((stream, _)) => {
				let mut handler = MasterMessageHandler::new(
					Context {
						sock_fd: -1,
						master: None,
					},
					stream,
				);
				let join_handle = tokio::spawn(async move {
					handler.start_handler();
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
