use tracing::info;

use super::IPCHandle;

pub struct MessageService<T: IPCHandle> {
    ipc_handle: T,
}

impl<T: IPCHandle> MessageService<T> {
    pub fn new(ipc_handle: T) -> Self {
        Self { ipc_handle }
    }

    pub fn run(&self) {
        info!("MessageService is running.");
        let message = "close";
        info!("Sending IPC command: {message}");
        self.ipc_handle.send(message.as_bytes());
    }
}
