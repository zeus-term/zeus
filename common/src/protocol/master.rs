use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
	SyncEnv(u32, Vec<EnvVar>),
	SyncTermios,
	SyncWinSize { row: u32, col: u32 },
	Forward(u32, Vec<u8>),
	AutoComplete(String),
	Init,
	Ack(i32),
	ForwardAck(u32),
	AckPty(String, Option<i32>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvVar {
	pub key: String,
	pub value: String,
}

pub trait HandleMessage {
	fn handle(&mut self, msg: Message) -> Message;
}
