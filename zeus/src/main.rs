pub mod core;

use crate::core::service::conn_handler::handle_conn;
use crate::core::service::shellmanager::ShellManager;
use common::constants::socket::HERMES_COMM;
use log::{error, info};
use nix::{
    pty::PtyMaster,
    sys::wait::waitpid,
    unistd::{fork, Pid},
};
use simple_logger::SimpleLogger;
use std::{
    io::{self},
    os::unix::net::UnixListener,
};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> io::Result<()> {
    SimpleLogger::new().env().init().unwrap();

    info!("Bootstrap process started");

    let _ = std::fs::remove_file(HERMES_COMM);
    info!("Cleaning up unix sockets");

    let listener = UnixListener::bind(HERMES_COMM)?;
    let mut ptymaster: Option<PtyMaster> = None;

    let mut children: Vec<Pid> = Vec::new();

    loop {
        info!("Started to listen on sock");
        match listener.accept() {
            Ok((socket, addr)) => {
                info!("New connection from {:?}", addr);

                let (send, recv) = oneshot::channel::<PtyMaster>();

                tokio::task::spawn(async move {
                    handle_conn(socket, send).await;
                });

                match recv.await {
                    Ok(master) => match unsafe { fork() } {
                        Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                            children.push(child);
                        }
                        Ok(nix::unistd::ForkResult::Child) => {
                            ptymaster = Some(master);
                            break;
                        }
                        Err(err) => {
                            error!("Error occured when creating a fork of zeus master: {}", err);
                        }
                    },
                    Err(err) => {
                        error!("Error occured when creating psuedo-terminal, {}", err);
                    }
                }
            }
            Err(err) => {
                error!("Failed to accept connection: {}", err);
            }
        }
    }

    if let Some(ptymaster) = ptymaster {
        info!("Starting the master side for pty: {:?}", ptymaster);
        let mut shell = ShellManager::new("/bin/zsh".to_string());
        shell.start();
        shell.write("nvim\n".as_bytes());
    } else {
        info!("Waiting all the children...");
        for child in children.iter() {
            waitpid(*child, None).unwrap();
        }
    }

    Ok(())
}
