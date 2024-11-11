pub mod client;
pub mod host;

pub trait IPCHandle {
    fn send(&self, data: &[u8]);
    fn read(&self) -> Vec<u8>;
    fn close(&self);
}
