use crate::core::service::shell::start_shell_subprocess;
use common::{
	err::Error,
	forwarder::{start_forwarder, FdForward},
};
use log::{error, info};
use nix::unistd::{fork, ForkResult, Pid};

pub enum ZForkResponse {
	Parent(Pid),
	Child,
}

pub fn fork_process(forwarders: &[FdForward], pts_path: &str) -> Result<ZForkResponse, Error> {
	match unsafe { fork() } {
		Ok(ForkResult::Parent { child }) => Ok(ZForkResponse::Parent(child)),
		Ok(nix::unistd::ForkResult::Child) => {
			info!("Starting shell process");
			start_shell_subprocess(pts_path);
			Ok(ZForkResponse::Child)
		}
		Err(err) => {
			error!("{}", err);
			Err(Error::ProcessForkError)
		}
	}
}
