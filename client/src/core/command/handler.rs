use std::{
	io::BufReader,
	os::{
		fd::{AsRawFd, FromRawFd},
		unix::net::UnixStream,
	},
};

use common::protocol::{base_handler::MessageHandler, message::Message};

use super::{ack_pty, context::Context};
use tokio::sync::oneshot::Sender;

pub struct ClientMessageHandler {
	ctx: Context,
	pid_tx: Option<Sender<i32>>,
	stream: UnixStream,
}

impl ClientMessageHandler {
	pub fn new(ctx: Context, stream: UnixStream) -> ClientMessageHandler {
		ClientMessageHandler {
			ctx,
			pid_tx: None,
			stream,
		}
	}
}

impl MessageHandler<Context> for ClientMessageHandler {
	fn handle(msg: Message, ctx: &Context) -> Option<Message> {
		match msg {
			Message::AckPty(pts_path, pid) => Some(ack_pty::handle(pts_path, pid, ctx)),
			_ => Some(Message::Ack(-1)),
		}
	}

	fn get_context(&mut self) -> &Context {
		&self.ctx
	}

	fn get_read_stream(&mut self) -> std::io::BufReader<UnixStream> {
		let sock_fd = self.stream.as_raw_fd();
		let unix_stream: UnixStream = unsafe { UnixStream::from_raw_fd(sock_fd) };

		BufReader::new(unix_stream)
	}

	fn get_write_stream(&mut self) -> &mut UnixStream {
		&mut self.stream
	}
}
