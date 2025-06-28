use tokio::sync::oneshot::Sender;

pub struct Context {
    pub sock_fd: i32,
    pub pid_tx: Sender<i32>
}
