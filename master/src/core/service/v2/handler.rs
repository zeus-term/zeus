use log::{error, info};
use std::os::unix::net::UnixListener;

use common::protocol::{base_handler::Context, base_handler::MessageHandler, master::Message};

pub fn handle(msg: Message, ctx: Context) -> Message {
	match msg {
		Message::Init => super::command::init::handle(msg, ctx),
		_ => Message::Ack(-1),
	}
}

pub fn handler(listener: UnixListener) {
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
