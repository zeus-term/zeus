use crate::core::service::shell::start_shell_subprocess;
use common::{
	err::Error,
	forwarder::{start_forwarder, FdForward},
};
use nix::unistd::{fork, ForkResult, Pid};

pub enum ZForkResponse {
	Parent(Pid),
	Child,
}

pub fn fork_process(forwarders: &[FdForward], pts_path: &str) -> Result<ZForkResponse, Error> {
	match unsafe { fork() } {
		Ok(ForkResult::Parent { child }) => {
			for fwd in forwarders {
				let FdForward { from, to } = *fwd;
				tokio::task::spawn_blocking(move || {
					start_forwarder(from, to);
				});
			}

			Ok(ZForkResponse::Parent(child))
		}
		Ok(nix::unistd::ForkResult::Child) => {
			start_shell_subprocess(pts_path);
			Ok(ZForkResponse::Child)
		}
		Err(_) => Err(Error::ProcessForkError),
	}
}
