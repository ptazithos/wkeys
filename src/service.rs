use relm4::RelmApp;

use crate::{
    layout::parse::LayoutDefinition,
    ui::{StyleAssets, UIMessage, UIModel},
};

pub trait KeyboardHandle {
    fn key_press(&mut self, key: evdev::Key);
    fn key_release(&mut self, key: evdev::Key);
    fn append_mod(&mut self, key: evdev::Key);
    fn remove_mod(&mut self, key: evdev::Key);
    fn append_lock(&mut self, key: evdev::Key);
    fn remove_lock(&mut self, key: evdev::Key);
}

pub struct AppService<M: KeyboardHandle + 'static> {
    ui_handle: RelmApp<UIMessage>,
    keyboard_handle: M,
    layout_definition: LayoutDefinition,
}

impl<M: KeyboardHandle + 'static> AppService<M> {
    pub fn new(keyboard_handle: M, layout_definition: LayoutDefinition) -> Self {
        let ui = RelmApp::new("net.pithos.wkeys");

        let css_str = StyleAssets::get_default_style_file();
        relm4::set_global_css(&css_str);
        
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
