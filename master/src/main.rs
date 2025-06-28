pub mod core;

use common::constants::socket::CLIENT_COMM;
use core::service::v2::handler::request_handler;
use core::utils::socket::cleanup_socket;
use log::info;
use simple_logger::SimpleLogger;
use std::{
	io::{self},
	os::unix::net::UnixListener,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> io::Result<()> {
	SimpleLogger::new().env().init().unwrap();

	info!("Bootstrap process started");

	cleanup_socket(CLIENT_COMM);

	let listener = UnixListener::bind(CLIENT_COMM)?;

	request_handler(listener);

	Ok(())
}
