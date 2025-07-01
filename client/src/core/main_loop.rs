use core::panic;
use std::{
	io::{self},
	os::unix::net::UnixStream,
};
use tokio::sync::mpsc::Receiver;
use tokio::{sync::mpsc, task};

use common::protocol::{base_handler::MessageHandler, message::Message};
use inotify::Inotify;

use super::{command::handler::ClientMessageHandler, init::get_term_state, term_state::TermState};
use crate::{core::command::context::Context, utils::buffer::handle_input};

const INIT_MSG: Message = Message::Init;

pub async fn start_input_handler(mut stream: UnixStream, mut state_chan: Receiver<TermState>) {
	let (mut io_handler, mut buffer, key_mapper) = get_term_state();
	let _ = io_handler.disable_line_buffering();
	let mut state = state_chan.recv().await.unwrap();
	let mut msg_handler = ClientMessageHandler::new(
		Context {
			pid_tx: None,
			sock_fd: None,
		},
		stream,
	);
	let _ = msg_handler.send_response(&INIT_MSG);

	while let Ok(data) = io_handler.read() {
		let mut keys: Vec<u8> = Vec::new();

		if let Ok(val) = state_chan.try_recv() {
			state = val;
		}

		match state {
			TermState::Forward => {
				// Just forward to the shell nothing crazye :)
				let msg: Vec<u8> = vec![data];
				let _ = msg_handler.send_response(&Message::Forward(1, msg));
			}
			TermState::Normal => {
				if !buffer.in_buf.is_empty() {
					keys.extend_from_slice(&buffer.in_buf);
				}

				keys.push(data);
				if let Ok(callback) = key_mapper.key_fn(&keys) {
					let data = handle_input(callback(), &mut buffer, &mut io_handler, false);
					if let Some(data) = data {
						let _ =
							msg_handler.send_response(&Message::Forward(data.len() as u32, data));
					}
				} else {
					buffer.flush_buffer();
				}
			}
		}
	}
}

pub async fn start_main_loop(sock: UnixStream) -> io::Result<()> {
	let stream = sock;
	let _inotity = match Inotify::init() {
		Ok(inotify) => inotify,
		Err(_) => {
			panic!("Error intiializing inotify");
		}
	};

	let (tx, wx) = mpsc::channel::<TermState>(1);
	let join = task::spawn(start_input_handler(stream, wx));
	tx.send(TermState::Normal).await;
	join.await;

	Ok(())
}
