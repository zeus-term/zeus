pub mod core;
pub mod platform;
pub mod utils;

use core::init::get_term_state;
use std::io::{self, Write};
use utils::buffer::handle_input;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (mutex_handler, mutex_buffer, key_mapper) = get_term_state();

    if let (Ok(mut handler), Ok(mut buffer)) =
        (mutex_handler.lock(), mutex_buffer.lock())
    {
        handler.disable_line_buffering()?;
        while let Ok(data) = handler.read() {
            let mut keys: Vec<u8> = Vec::new();

            if !buffer.in_buf.is_empty() {
                keys.extend_from_slice(&buffer.in_buf);
            }

            keys.push(data);

            if let Ok(callback) = key_mapper.key_fn(&keys) {
                let _ = handle_input(callback(), &mut buffer, &mut handler);
            } else {
                buffer.flush_buffer();
            }
            io::stdout().flush()?;
        }
    } else {
        panic!("IO is blocked cannot start hermes ;(");
    }
    Ok(())
}
