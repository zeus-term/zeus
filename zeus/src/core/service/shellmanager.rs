use std::{
    fmt::Display,
    io::{Read, Write},
    process::{ChildStderr, ChildStdin, ChildStdout, Command, Stdio},
};

use log::warn;

pub enum ShellStatus {
    NotStarted,
    Running,
    Terminated { exit_code: u16 },
    StartupError { reason: String },
}

#[derive(Debug, Clone)]
pub struct ChildWriteError {}

#[derive(Debug, Clone)]
pub struct ChildReadError {
    stream: u8,
}

impl Display for ChildWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error writing to child process")
    }
}

impl Display for ChildReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error reading from child process, stream: {}",
            match self.stream {
                1 => "Stdout",
                2 => "Stderr",
                _ => "Unknown",
            }
        )
    }
}

pub struct ShellManager {
    program: String,
    status: ShellStatus,
    pid: Option<u32>,
    child_stdin: Option<ChildStdin>,
    child_stdout: Option<ChildStdout>,
    child_stderr: Option<ChildStderr>,
}

impl ShellManager {
    pub fn new(program: String) -> ShellManager {
        ShellManager {
            program,
            status: ShellStatus::NotStarted,
            pid: None,
            child_stdin: None,
            child_stdout: None,
            child_stderr: None,
        }
    }

    pub fn start(&mut self) {
        match Command::new(self.program.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(shell_process) => {
                self.child_stdin = shell_process.stdin;
                self.child_stdout = shell_process.stdout;
                self.child_stderr = shell_process.stderr;
            }
            Err(_) => {
                self.status = ShellStatus::StartupError {
                    reason: "Error running the binary".to_string(),
                }
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, ChildWriteError> {
        if let Some(stdin) = self.child_stdin.as_mut() {
            match stdin.write(data) {
                Ok(_) => Ok(data.len()),
                Err(_) => Err(ChildWriteError {}),
            }
        } else {
            warn!("Error writing to shell process before startup or the shell terminated unexpectedly");
            Err(ChildWriteError {})
        }
    }

    pub fn read_stdout(&mut self, buffer: &mut [u8]) -> Result<usize, ChildReadError> {
        if let Some(stdout) = self.child_stdout.as_mut() {
            match stdout.read_exact(buffer) {
                Ok(_) => Ok(buffer.len()),
                Err(_) => Err(ChildReadError { stream: 1 }),
            }
        } else {
            Err(ChildReadError { stream: 1 })
        }
    }

    pub fn read_stderr(&mut self, buffer: &mut [u8]) -> Result<usize, ChildReadError> {
        if let Some(stderr) = self.child_stderr.as_mut() {
            match stderr.read_exact(buffer) {
                Ok(_) => Ok(buffer.len()),
                Err(_) => Err(ChildReadError { stream: 2 }),
            }
        } else {
            Err(ChildReadError { stream: 2 })
        }
    }

    pub fn program(&self) -> &String {
        &self.program
    }

    pub fn status(&self) -> &ShellStatus {
        &self.status
    }

    pub fn pid(&self) -> &Option<u32> {
        &self.pid
    }
}

impl Default for ShellManager {
    fn default() -> Self {
        Self::new("/bin/bash".to_string())
    }
}
