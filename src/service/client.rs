use tracing::info;

pub struct MessageService;

impl MessageService {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        info!("MessageService is running.");
    }
}
