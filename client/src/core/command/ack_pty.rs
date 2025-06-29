use common::protocol::message::Message;

use super::context::Context;

pub fn handle(_pts_path: String, pid: Option<i32>, ctx: &Context) -> Message {
	Message::Ack(1)
}
