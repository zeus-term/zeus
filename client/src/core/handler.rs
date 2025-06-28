use common::protocol::{base_handler::Context, message::Message};

use super::command::ack_pty;

pub fn handle(msg: Message, ctx: &Context) -> Option<Message> {
	match msg {
		Message::AckPty(pts_path, pid) => Some(ack_pty::handle(pts_path, pid, ctx)),
		_ => Some(Message::Ack(-1)),
	}
}
