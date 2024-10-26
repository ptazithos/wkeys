use relm4::RelmApp;

use crate::{
    layout::parse::LayoutDefinition,
    ui::{UIMessage, UIModel},
};

pub trait KeyboardHandle {
    fn key_press(&mut self, key: evdev::Key);
    fn key_release(&mut self, key: evdev::Key);
    fn set_mod(&mut self, key: evdev::Key);
    fn remove_mod(&mut self);
}

pub struct AppService<M: KeyboardHandle + 'static> {
    ui_handle: RelmApp<UIMessage>,
    keyboard_handle: M,
    layout_definition: LayoutDefinition,
}

impl<M: KeyboardHandle + 'static> AppService<M> {
    pub fn new(keyboard_handle: M, layout_definition: LayoutDefinition) -> Self {
        let ui = RelmApp::new("net.pithos.wkeys");
        Self {
            ui_handle: ui,
            keyboard_handle,
            layout_definition,
        }
    }

    pub fn run(self) {
        self.ui_handle
            .run::<UIModel>((Box::new(self.keyboard_handle), self.layout_definition));
    }
}
