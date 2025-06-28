use std::os::fd::AsFd;

use common::protocol::{base_handler::Context, message::Message};
use nix::unistd::write;

pub fn handle(size: u32, data: Vec<u8>, ctx: &Context) -> Message {
	match &ctx.master {
		Some(pty) => {
			let _ = write(pty.as_fd(), data.as_slice());
			Message::Ack(size as i32)
		}
		None => Message::Ack(-1),
	}
}
