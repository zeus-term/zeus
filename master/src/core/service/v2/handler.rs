use common::protocol::{base_handler::Context, base_handler::MessageHandler, message::Message};
use log::{error, info};
use std::os::unix::net::UnixListener;

pub fn handle(msg: Message, ctx: &Context) -> Option<Message> {
	match msg {
		Message::Init => Some(super::command::init::handle(msg, ctx)),
		Message::Forward(size, data) => Some(super::command::forward::handle(size, data, ctx)),
		Message::Ack(_val) => None,
		_ => Some(Message::Ack(-1)),
	}
}

pub fn request_handler(listener: UnixListener) {
	loop {
		info!("Starting v2 master listener...");
		match listener.accept() {
			Ok((stream, _)) => {
				let mut handler = MessageHandler::new(stream, handle);
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
