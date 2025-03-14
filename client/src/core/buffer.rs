use std::{fmt, usize};

type Command = Vec<u8>;

#[derive(Debug, Clone)]
pub struct ExecutionFailed {
	command: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CanNotGoBackError;

#[derive(Debug, Clone)]
pub struct CannotBufferData;

pub struct Buffer {
	caret: usize,
	history: Vec<Command>,
	history_ref: usize,

	pub active: Command,
	pub in_buf: Vec<u8>,
}

pub enum BufChangeInstruction {
	Buffer(u8),
	StepForward,
	StepBackward,
}

impl fmt::Display for ExecutionFailed {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Failed to execute command: {:?}",
			String::from_utf8(self.command.clone())
		)
	}
}

impl fmt::Display for CannotBufferData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Cannot buffer data",)
	}
}

impl Default for Buffer {
	fn default() -> Self {
		Self::new()
	}
}

impl Buffer {
	pub fn new() -> Buffer {
		Buffer {
			caret: 0,
			active: Vec::new(),
			in_buf: Vec::new(),

			history: Vec::new(),
			history_ref: 0,
		}
	}

	pub fn push_active_command(&mut self) -> Result<(), ExecutionFailed> {
		self.history.push(self.active.clone());

		self.active.clear();
		self.caret = 0;
		self.history_ref = self.history.len();
		Ok(())
	}

	pub fn step_backward(&mut self) -> Result<(), CanNotGoBackError> {
		if self.history.is_empty() || self.history_ref == 0 {
			return Err(CanNotGoBackError {});
		}

		self.active = self.history.get(self.history_ref - 1).unwrap().to_vec();
		self.history_ref -= 1;
		self.caret = self.active.len();

		Ok(())
	}

	pub fn step_forward(&mut self) -> Result<(), CanNotGoBackError> {
		Ok(())
	}

	pub fn buf_data(&mut self, data: u8) -> Result<(), CannotBufferData> {
		if self.in_buf.len() == 512 {
			return Err(CannotBufferData {});
		}

		self.in_buf.push(data);

		Ok(())
	}

	pub fn flush_buffer(&mut self) {
		self.in_buf.clear();
	}

	pub fn insert_into_active(&mut self, data: u8) {
		if self.caret == self.active.len() {
			self.active.push(data);
		} else {
			self.active.insert(self.caret, data);
		}
		self.caret += 1;
	}

	pub fn move_forward_caret(&mut self) {
		if self.caret == self.active.len() {
			return;
		}
		self.caret += 1;
	}

	pub fn move_backward_caret(&mut self) {
		if self.caret == 0 {
			return;
		}

		self.caret -= 1;
	}

	pub fn chars_ahead_caret(&self) -> &[u8] {
		&self.active[self.caret..]
	}

	pub fn backspace_active_buffer(&mut self) {
		if self.caret == 0 {
			return;
		}
		self.active.remove(self.caret - 1);
		self.caret -= 1;
	}
}
