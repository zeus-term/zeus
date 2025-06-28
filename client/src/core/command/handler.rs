use common::protocol::message::Message;

use super::{ack_pty, context::Context};

pub fn handle(msg: Message, ctx: &Context) -> Option<Message> {
	match msg {
		Message::AckPty(pts_path, pid) => Some(ack_pty::handle(pts_path, pid, ctx)),
		_ => Some(Message::Ack(-1)),
	}
}
