use nix::pty::PtyMaster;

pub struct Context {
	pub sock_fd: i32,
	pub master: Option<PtyMaster>,
}
