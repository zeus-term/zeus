use core::panic;
use std::{
	io::{self, Write},
	os::{
		fd::{AsRawFd, BorrowedFd},
		unix::net::UnixStream,
	},
};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use common::protocol::{base_handler::MessageHandler, message::Message, utils::raw_message};
use inotify::Inotify;

use super::{init::get_term_state, term_state::TermState};

fn init(stream: &mut UnixStream) {
	let msg = Message::Init;
	let _ = stream.write_all(raw_message(msg).unwrap().as_slice());
}

pub async fn start_input_handler(fd: BorrowedFd<'_>, mut state_chan: Receiver<TermState>) {
	let (mut io_handler, mut buffer, key_mapper) = get_term_state();
	let _ = io_handler.disable_line_buffering();
	let mut state = state_chan.recv().await.unwrap();
	let mut handler = MessageHandler::new(streamm, handle);

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
	let mut stream = sock;
	init(&mut stream);
	let fd = stream.as_raw_fd();
	let _inotity = match Inotify::init() {
		Ok(inotify) => inotify,
		Err(_) => {
			panic!("Error intiializing inotify");
		}
	};
	let (tx, wx) = mpsc::channel::<TermState>(1);

	Ok(())
}
