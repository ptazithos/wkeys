use tracing::info;

use crate::utils::ProgramArgs;

use super::IPCHandle;

pub struct MessageService<T: IPCHandle> {
    ipc_handle: T,
    args: ProgramArgs,
}

impl<T: IPCHandle> MessageService<T> {
    pub fn new(ipc_handle: T, args: ProgramArgs) -> Self {
        Self { ipc_handle, args }
    }

    pub fn run(&self) {
        info!("MessageService is running.");
        if self.args.message.is_some() {
            let message = "close";
            info!("Sending IPC command: {message}");
            self.ipc_handle.send(message.as_bytes());
        }
    }
}
