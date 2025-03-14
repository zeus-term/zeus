use common::{
	err::Error,
	forwarder::{start_forwarder, FdForward},
};
use nix::unistd::{fork, ForkResult, Pid};

pub enum ZForkResponse {
	Parent(Pid),
	Child,
}

pub fn fork_process(forward1: FdForward, forward2: FdForward) -> Result<ZForkResponse, Error> {
	match unsafe { fork() } {
		Ok(ForkResult::Parent { child }) => Ok(ZForkResponse::Parent(child)),
		Ok(nix::unistd::ForkResult::Child) => {
			tokio::task::spawn_blocking(move || {
				start_forwarder(forward1.from, forward1.to);
			});

			tokio::task::spawn_blocking(move || {
				start_forwarder(forward2.from, forward2.to);
			});

			Ok(ZForkResponse::Child)
		}
		Err(_) => Err(Error::ProcessForkError),
	}
}
