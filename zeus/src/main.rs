pub mod core;

use crate::core::constants::sock::HERMES_COMM;
use crate::core::service::conn_handler::handle_conn;
use log::{error, info};
use simple_logger::SimpleLogger;
use std::{
    io::{self},
    os::unix::net::UnixListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    SimpleLogger::new().env().init().unwrap();

    info!("Bootstrap process started");

    let _ = std::fs::remove_file(HERMES_COMM);
    info!("Cleaning up unix sockets");

    let listener = UnixListener::bind(HERMES_COMM)?;

    loop {
        info!("Started to listen on sock");
        match listener.accept() {
            Ok((socket, addr)) => {
                info!("New connection from {:?}", addr);
                tokio::task::spawn(async move {
                    handle_conn(socket).await;
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {:?}", e);
            }
        }
    }
}
