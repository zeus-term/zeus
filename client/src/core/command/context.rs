use tokio::sync::oneshot::Sender;

pub struct Context {
	pub sock_fd: Option<i32>,
	pub pid_tx: Option<Sender<i32>>,
}
