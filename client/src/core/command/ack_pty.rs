use common::protocol::{base_handler::Context, message::Message};

pub fn handle(_pts_path: String, pid: Option<i32>, ctx: &Context) -> Message {
	Message::Ack(1)
}
