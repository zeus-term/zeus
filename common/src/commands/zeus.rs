use super::traits::Cmd;

/// Commands that zeus module can handle
pub enum Command {
    Exec { binary: String, args: Vec<String> },
    ShowCompletions { partial_cmd: String },
    CreatePty,
}

impl Cmd for Command {
    fn validate(&self) -> bool {
        false
    }
}
