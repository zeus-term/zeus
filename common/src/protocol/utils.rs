use super::master::Message;
use crate::{constants::character::general_ascii_chars::NEWLINE, err::Error as ZError};
use postcard::to_slice;

pub fn raw_message(msg: Message) -> Result<Vec<u8>, ZError> {
	let mut buf: [u8; 32] = [0; 32];

	let result = to_slice(&msg, &mut buf);

	match result {
		Ok(arr) => Ok([arr.to_vec().as_slice(), &[NEWLINE]].concat()),
		Err(_) => Err(ZError::SerializationError),
	}
}
