use relm4::RelmApp;
use tracing::info;

use crate::{
    layout::parse::LayoutDefinition,
    ui::{UIMessage, UIModel},
};

use super::IPCHandle;

pub trait KeyboardHandle {
    fn key_press(&mut self, key: evdev::KeyCode);
    fn key_release(&mut self, key: evdev::KeyCode);

    fn append_mod(&mut self, key: evdev::KeyCode);
    fn remove_mod(&mut self, key: evdev::KeyCode);

    fn append_lock(&mut self, key: evdev::KeyCode);
    fn remove_lock(&mut self, key: evdev::KeyCode);

    fn destroy(&mut self);
}

pub struct AppService<M: KeyboardHandle + 'static, N: IPCHandle + Send + 'static> {
    ui_handle: RelmApp<UIMessage>,
    keyboard_handle: M,
    ipc_handle: N,
    layout_definition: LayoutDefinition,
}

impl<M: KeyboardHandle + 'static, N: IPCHandle + Send + 'static> AppService<M, N> {
    pub fn new(
        keyboard_handle: M,
        ipc_handle: N,
        layout_definition: LayoutDefinition,
        styles: String,
    ) -> Self {
        let ui = RelmApp::new("net.pithos.wkeys");

        relm4::set_global_css(&styles);

        Self {
            ui_handle: ui,
            keyboard_handle,
            ipc_handle: ipc_handle,
            layout_definition,
        }
    }

    pub fn run(self) {
        info!("Starting UI.");
        self.ui_handle.with_args(vec![]).run::<UIModel>((
            Box::new(self.keyboard_handle),
            Box::new(self.ipc_handle),
            self.layout_definition,
        ));
    }
}
