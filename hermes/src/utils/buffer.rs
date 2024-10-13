use super::term;
use super::term::{DataWriteOrder, DisplaceDirection, PostCaretPosition};
use crate::core::{buffer::Buffer, io::IOHandler, key_mapper::KeypressAction};
use common::constants::character::general_ascii_chars::NEWLINE;
use common::constants::character::{
	general_ascii_chars, printable_ascii_characters::SPACE, triplet_char_actions,
};
use std::iter;

/// Utility function for buffer data structure related functions for performing crud on it
pub fn handle_input(
	action: KeypressAction,
	buffer: &mut Buffer,
	handler: &mut IOHandler,
	echo: bool,
) -> Option<Vec<u8>> {
	match action {
		KeypressAction::Return(byte) => {
			match byte {
				general_ascii_chars::ENTER => {
					// TODO: Properly handle errors
					let _ = buffer.push_active_command();
					handler.disable_line_buffering().unwrap();
					let _ = handler.write_str(&[NEWLINE]);
				}
				general_ascii_chars::TAB => {
					// TODO: Send the master to show auto completion
				}
				general_ascii_chars::BACKSPACE => {
					buffer.backspace_active_buffer();
					let mut pending_writable_chars: Vec<u8> = Vec::new();
					pending_writable_chars.extend_from_slice(buffer.chars_ahead_caret());
					pending_writable_chars.push(SPACE);

					if echo {
						term::write(
							Some(&pending_writable_chars),
							1,
							PostCaretPosition::Return,
							DisplaceDirection::Left,
							DataWriteOrder::PostCaretMovement,
						);
					}
				}
				_ => {
					buffer.insert_into_active(byte);
					if echo {
						term::write(
							Some(&[byte]),
							0,
							PostCaretPosition::Stay,
							DisplaceDirection::None,
							DataWriteOrder::PostCaretMovement,
						);

						let pending_writable_chars = buffer.chars_ahead_caret();
						if !pending_writable_chars.is_empty() {
							term::write(
								Some(pending_writable_chars),
								0,
								PostCaretPosition::Return,
								DisplaceDirection::None,
								DataWriteOrder::PreCaretMovement,
							);
						}
					}
				}
			}
			buffer.flush_buffer();
			return Some(vec![byte]);
		}
		KeypressAction::Buffer(byte) => {
			if buffer.buf_data(byte).is_err() {
				buffer.flush_buffer();
			}
		}
		KeypressAction::Signal(signal) => {
			// TODO: handle general shell signals that can be recieved from input
		}
		KeypressAction::Action(action) => {
			match action {
				triplet_char_actions::Chars::Up => {
					if echo {
						term::write(
							Some(
								&iter::repeat(SPACE)
									.take(buffer.active.len())
									.collect::<Vec<u8>>(),
							),
							buffer.active.len(),
							PostCaretPosition::Stay,
							DisplaceDirection::Left,
							DataWriteOrder::PostCaretMovement,
						);
						term::write(
							None,
							buffer.active.len(),
							PostCaretPosition::Stay,
							DisplaceDirection::Left,
							DataWriteOrder::NoData,
						);
						let _ = buffer.step_backward();
						term::write(
							Some(&buffer.active),
							0,
							PostCaretPosition::Stay,
							DisplaceDirection::None,
							DataWriteOrder::PostCaretMovement,
						);
					}
					return Some(triplet_char_actions::UP.to_vec());
				}
				triplet_char_actions::Chars::Down => {
					// TODO: handle forward movement in history
				}
				triplet_char_actions::Chars::Left => {
					if echo {
						buffer.move_backward_caret();
						term::write(
							None,
							1,
							PostCaretPosition::Stay,
							DisplaceDirection::Left,
							DataWriteOrder::NoData,
						);
					}
					return Some(triplet_char_actions::LEFT.to_vec());
				}
				triplet_char_actions::Chars::Right => {
					if echo {
						buffer.move_forward_caret();
						term::write(
							None,
							1,
							PostCaretPosition::Stay,
							DisplaceDirection::Right,
							DataWriteOrder::NoData,
						);
					}
					return Some(triplet_char_actions::RIGHT.to_vec());
				}
				// For now I don't care about rest of the actions
				_ => {}
			}
			buffer.flush_buffer();
		}
		KeypressAction::MasterCommand(cmd) => {}
	}
	None
}
