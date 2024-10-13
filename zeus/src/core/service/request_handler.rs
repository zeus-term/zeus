use std::os::unix::net::UnixStream;

use nix::pty::PtyMaster;
use tokio::sync::oneshot::{channel, Receiver};

use super::conn_handler::init_handle_conn;

pub async fn serve_request(socket: UnixStream) -> (Receiver<PtyMaster>, Receiver<UnixStream>) {
	let (send, recv_pty) = channel::<PtyMaster>();
	let (send_stream, recv_stream) = channel::<UnixStream>();

	// TODO: handle err
	let _ = tokio::task::spawn(async move {
		init_handle_conn(socket, send, send_stream).await;
	})
	.await;

	(recv_pty, recv_stream)
}
