use super::master::{Message, Response};
use crate::err::Error as ZError;
use postcard::to_slice;

pub fn raw_message(msg: Message) -> Result<Vec<u8>, ZError> {
	let mut buf: [u8; 32] = [0; 32];

	let result = to_slice(&msg, &mut buf);

	match result {
		Ok(arr) => Ok(arr.to_vec()),
		Err(_) => Err(ZError::SerializationError),
	}
}

pub fn raw_response(res: Response) -> Result<Vec<u8>, ZError> {
	let mut buf: [u8; 32] = [0; 32];

	let result = to_slice(&res, &mut buf);

	match result {
		Ok(arr) => Ok(arr.to_vec()),
		Err(_) => Err(ZError::SerializationError),
	}
}
