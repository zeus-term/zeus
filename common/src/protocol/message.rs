use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
	/// Synchronize environment variables between client and PTY
	/// - First parameter: client ID
	/// - Second parameter: list of environment variables to sync
	SyncEnv(u32, Vec<EnvVar>),

	/// Request to synchronize terminal I/O settings (termios)
	SyncTermios,

	/// Synchronize terminal window size
	/// - row: Number of rows in the terminal
	/// - col: Number of columns in the terminal
	SyncWinSize { row: u32, col: u32 },

	/// Forward data to the shell
	/// - First parameter: client ID
	/// - Second parameter: data bytes to forward
	Forward(u32, Vec<u8>),

	/// Trigger auto-completion for the given input string
	AutoComplete(String),

	/// Acknowledgment for a command
	/// - Contains status code (negative for errors)
	Ack(i32),

	/// Acknowledgment for shell forwarding request
	/// - Contains client ID that requested forwarding
	ForwardAck(u32),

	/// Initialization message
	Init,

	/// Acknowledgment for initialization
	/// - First parameter: PTY identifier
	/// - Second parameter: Optional status code (None if successful)
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
