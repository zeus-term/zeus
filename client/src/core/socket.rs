use common::constants::{socket::CLIENT_COMM, STDOUT_FILENO};
use nix::unistd::dup;
use std::os::{
	fd::{AsRawFd, OwnedFd},
	unix::net::UnixStream,
};
use std::thread;

pub fn connect_master() -> UnixStream {
	UnixStream::connect(CLIENT_COMM).unwrap()
}
