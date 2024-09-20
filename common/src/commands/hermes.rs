use super::traits::Cmd;

/// Commands that hermes module can handle
pub enum Command {
	Output(Vec<u8>),
	CommandSelected(Vec<u8>),
}

impl Cmd for Command {
	fn validate(&self) -> bool {
		false
	}
}
