use core::panic;
use std::{
	io::{self},
	os::unix::net::UnixStream,
};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use common::protocol::{base_handler::MessageHandler, message::Message};
use inotify::Inotify;

use super::{command::handler::ClientMessageHandler, init::get_term_state, term_state::TermState};
use crate::core::command::context::Context;

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
				// TODO: handle to forward the data to the shell
			}
			TermState::Normal => {
				// TODO: intercept the data and handle the auto completion logic
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
	start_input_handler(stream, wx).await;

	Ok(())
}
