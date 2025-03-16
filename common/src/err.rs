use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
	MessageParsingError,
	SocketReadError,
	PtyCreationError,
	ProcessForkError,
	ConnectionRefusedError,
	SerializationError,
	DeserializationError,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Error::*;

		write!(
			f,
			"{}",
			match self {
				MessageParsingError => "Error parsing the message",
				SocketReadError => "Error reading data from socket",
				PtyCreationError => "Error creating pty",
				ProcessForkError => "Error forking process",
				ConnectionRefusedError => "Error accepting connections",
				SerializationError => "Message serialization error",
				DeserializationError => "Message deserialization error",
			}
		)
	}
}
