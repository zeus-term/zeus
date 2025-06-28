use common::constants::socket::CLIENT_COMM;
use std::os::unix::net::UnixStream;

pub fn connect_master() -> UnixStream {
	UnixStream::connect(CLIENT_COMM).unwrap()
}
