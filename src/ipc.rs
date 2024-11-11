use std::os::unix::net::UnixListener;

use tracing::info;

static UNIX_SOCKET_NAME: &str = "net.pithos.wkeyb";

pub struct IPCHandle {
    socket: Option<UnixListener>,
}

impl IPCHandle {
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
        std::fs::remove_file(UNIX_SOCKET_NAME).unwrap();
    }
}

impl Drop for IPCHandle {
    fn drop(&mut self) {
        if let Some(_) = self.socket.take() {
            IPCHandle::clean_up();
        }
    }
}
