use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
};

use tracing::{error, info};

use crate::service::IPCHandle;

static UNIX_SOCKET_NAME: &str = "/tmp/net.pithos.wkeys";

pub struct IPC {
    socket: Option<UnixListener>,
}

impl IPC {
    pub fn init() -> Self {
        match UnixListener::bind(UNIX_SOCKET_NAME) {
            Ok(listener) => {
                info!("Running as IPC server.");
                Self {
                    socket: Some(listener),
                }
            }
            Err(e) => {
                if e.raw_os_error().unwrap_or_default() == 98 {
                    info!("Another instance is already running.");
                    Self { socket: None }
                } else {
                    panic!("Failed to bind to socket: {}", e)
                }
            }
        }
    }

    pub fn is_single_instance(&self) -> bool {
        self.socket.is_some()
    }

    pub fn clean_up() {
        info!("Cleaning up IPC socket.");
        match std::fs::remove_file(UNIX_SOCKET_NAME) {
            Err(e) => {
                if e.raw_os_error().unwrap_or_default() != 2 {
                    error!("Failed to remove IPC socket: {}", e);
                }
            }
            _ => {}
        }
    }
}

impl Drop for IPC {
    fn drop(&mut self) {
        IPC::clean_up();
    }
}

impl IPCHandle for IPC {
    fn send(&self, data: &[u8]) {
        let mut stream = UnixStream::connect(UNIX_SOCKET_NAME).unwrap();
        stream.write_all(data).unwrap();
    }

    fn read(&self) -> Vec<u8> {
        if let Some(listener) = &self.socket {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut data = Vec::new();
                    stream.read_to_end(&mut data).unwrap();
                    return data;
                }
                Err(e) => {
                    info!("Failed to accept connection: {}", e);
                    return self.read();
                }
            }
        }
        vec![]
    }
}
