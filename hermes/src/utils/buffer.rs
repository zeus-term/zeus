use std::io;

use super::term;
use super::term::{DataWriteOrder, DisplaceDirection, PostCaretPosition};
use crate::core::{buffer::Buffer, io::IOHandler, key_mapper::KeypressAction};
use common::constants::character::{
    general_ascii_chars, printable_ascii_characters::SPACE,
    triplet_char_actions,
};
use std::iter;

/// Utility function for buffer data structure related functions for performing crud on it
pub fn handle_input(
    action: KeypressAction,
    buffer: &mut Buffer,
    handler: &mut IOHandler,
) -> io::Result<()> {
    match action {
        KeypressAction::Return(byte) => {
            match byte {
                general_ascii_chars::ENTER => {
                    // TODO: Submit the command to the master
                    handler.enable_line_buffering()?;
                    println!(
                        "\n{}",
                        String::from_utf8(buffer.active.clone()).unwrap()
                    );
                    // TODO: Properly handle errors
                    let _ = buffer.push_active_command();
                    handler.disable_line_buffering()?;
                }
                general_ascii_chars::TAB => {
                    // TODO: Send the master to show auto completion
                }
                general_ascii_chars::BACKSPACE => {
                    buffer.backspace_active_buffer();
                    let mut pending_writable_chars: Vec<u8> = Vec::new();
                    pending_writable_chars
                        .extend_from_slice(buffer.chars_ahead_caret());
                    pending_writable_chars.push(SPACE);

                    term::write(
                        Some(&pending_writable_chars),
                        1,
                        PostCaretPosition::Return,
                        DisplaceDirection::Left,
                        DataWriteOrder::PostCaretMovement,
                    );
                }
                _ => {
                    buffer.insert_into_active(byte);
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
            buffer.flush_buffer();
        }
        KeypressAction::Buffer(byte) => {
            if buffer.buf_data(byte).is_err() {
                buffer.flush_buffer();
            }
        }
        KeypressAction::Signal(signal) => {
            buffer.flush_buffer();
        }
        KeypressAction::Action(action) => {
            match action {
                triplet_char_actions::Chars::Up => {
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
                triplet_char_actions::Chars::Down => {
                    println!();
                }
                triplet_char_actions::Chars::Left => {
                    buffer.move_backward_caret();
                    term::write(
                        None,
                        1,
                        PostCaretPosition::Stay,
                        DisplaceDirection::Left,
                        DataWriteOrder::NoData,
                    );
                }
                triplet_char_actions::Chars::Right => {
                    buffer.move_forward_caret();
                    term::write(
                        None,
                        1,
                        PostCaretPosition::Stay,
                        DisplaceDirection::Right,
                        DataWriteOrder::NoData,
                    );
                }
                // For now I don't care about rest of the actions
                _ => {}
            }
            buffer.flush_buffer();
        }
        KeypressAction::MasterCommand(cmd) => {}
    }
    Ok(())
}
