use std::{fmt, io};

use crate::utils::stack::Stack;

type Command = Vec<u8>;

#[derive(Debug, Clone)]
pub struct ExecutionFailed {
    command: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CanNotGoBackError;

pub struct Buffer {
    stack: Stack<Command>,
    active: Vec<u8>,
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

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            stack: Stack::new(),
            active: Vec::new(),
        }
    }

    pub fn step_forward(
        &mut self,
        callback: fn() -> io::Result<()>,
    ) -> Result<(), ExecutionFailed> {
        self.stack.push(self.active.clone());

        match callback() {
            Ok(_) => Ok(()),
            Err(_) => Err(ExecutionFailed {
                command: self.active.clone(),
            }),
        }
    }

    pub fn step_backward(&mut self) -> Result<(), CanNotGoBackError> {
        let top = self.stack.pop();

        match top {
            Some(data) => {
                self.active = data;
                Ok(())
            }
            None => Err(CanNotGoBackError {}),
        }
    }
}
