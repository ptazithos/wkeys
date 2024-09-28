use relm4::RelmApp;
use tracing::error;

use crate::ui::{UIMessage, UIModel};

pub trait KeyboardHandle {
    fn key_press(&mut self, key: evdev::Key);
    fn key_release(&mut self, key: evdev::Key);
}

pub struct AppService<M: KeyboardHandle + 'static> {
    ui_handle: RelmApp<UIMessage>,
    keyboard_handle: Option<M>,
}

impl<M: KeyboardHandle + 'static> AppService<M> {
    pub fn new(keyboard_handle: M) -> Self {
        let ui = RelmApp::new("net.pithos.wkeys");
        Self {
            ui_handle: ui,
            keyboard_handle: Some(keyboard_handle),
        }
    }

    pub fn run(mut self) {
        if let Some(keyboard_handle) = self.keyboard_handle.take() {
            self.ui_handle.run::<UIModel>(Box::new(keyboard_handle));
        } else {
            error!("No keyboard handle");
        }
    }
}
