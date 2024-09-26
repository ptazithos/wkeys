use relm4::RelmApp;
use tracing::error;

use crate::ui::{UIMessage, UIModel};

pub trait KeyboardHandle {
    fn key_press(&mut self, key: evdev::Key);
    fn key_release(&mut self, key: evdev::Key);
}

pub struct AppService {
    ui_handle: RelmApp<UIMessage>,
    keyboard_handle: Option<Box<dyn KeyboardHandle>>,
}

impl AppService {
    pub fn new(keyboard_handle: Box<dyn KeyboardHandle>) -> Self {
        let ui = RelmApp::new("net.pithos.wkeys");
        Self {
            ui_handle: ui,
            keyboard_handle: Some(keyboard_handle),
        }
    }

    pub fn run(mut self) {
        if let Some(keyboard_handle) = self.keyboard_handle.take() {
            self.ui_handle.run::<UIModel>(keyboard_handle);
        } else {
            error!("No keyboard handle");
        }
    }
}
