#[derive(Clone, Copy)]
pub enum PostCaretPosition {
	Return,
	Stay,
}

#[derive(Clone, Copy)]
pub enum DisplaceDirection {
	Left,
	None,
	Right,
}

#[derive(Clone, Copy)]
pub enum DataWriteOrder {
	PostCaretMovement,
	NoData,
	PreCaretMovement,
}

macro_rules! move_left {
	($value: expr) => {
		format_args!("\x1B[{}D", $value)
	};
}

macro_rules! move_right {
	($value: expr) => {
		format_args!("\x1B[{}C", $value)
	};
}

pub fn write(
	data: Option<&[u8]>,
	delta: usize,
	post_operation: PostCaretPosition,
	direction: DisplaceDirection,
	order: DataWriteOrder,
) {
	if let (DataWriteOrder::PreCaretMovement, Some(bytes)) = (order, data) {
		for byte in bytes.iter() {
			print!("{}", *byte as char);
		}
	}

	match direction {
		DisplaceDirection::Left => {
			print!("{}", move_left!(delta));
		}
		DisplaceDirection::Right => {
			print!("{}", move_right!(delta));
		}
		DisplaceDirection::None => {}
	}

	if let (DataWriteOrder::PostCaretMovement, Some(bytes)) = (order, data) {
		for byte in bytes.iter() {
			print!("{}", *byte as char);
		}
	}

	if let (PostCaretPosition::Return, Some(bytes)) = (post_operation, data) {
		print!("{}", move_left!(bytes.len()));
	}
}
