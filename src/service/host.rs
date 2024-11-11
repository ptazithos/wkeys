use std::{sync::mpsc::channel, thread};

use relm4::RelmApp;
use tracing::info;

use crate::{
    layout::parse::LayoutDefinition,
    ui::{StyleAssets, UIMessage, UIModel},
};

use super::IPCHandle;

pub trait KeyboardHandle {
    fn key_press(&mut self, key: evdev::Key);
    fn key_release(&mut self, key: evdev::Key);

    fn append_mod(&mut self, key: evdev::Key);
    fn remove_mod(&mut self, key: evdev::Key);

    fn append_lock(&mut self, key: evdev::Key);
    fn remove_lock(&mut self, key: evdev::Key);

    fn destroy(&mut self);
}

pub struct AppService<M: KeyboardHandle + 'static, N: IPCHandle + Send + 'static> {
    ui_handle: RelmApp<UIMessage>,
    keyboard_handle: M,
    ipc_handle: N,
    layout_definition: LayoutDefinition,
}

impl<M: KeyboardHandle + 'static, N: IPCHandle + Send + 'static> AppService<M, N> {
    pub fn new(keyboard_handle: M, ipc_handle: N, layout_definition: LayoutDefinition) -> Self {
        let ui = RelmApp::new("net.pithos.wkeys");

        let css_str = StyleAssets::get_default_style_file();
        relm4::set_global_css(&css_str);

        Self {
            ui_handle: ui,
            keyboard_handle,
            ipc_handle: ipc_handle,
            layout_definition,
        }
    }

    pub fn run(self) {
        let (sender, receiver) = channel();
        let ipc_handle = self.ipc_handle;
        thread::spawn(move || loop {
            info!("Starting IPC server.");

            loop {
                let res = ipc_handle.read();
                if let Ok(message) = String::from_utf8(res) {
                    info!("Received IPC message: {}", message);
                    match message.as_str() {
                        "close" => {
                            sender.send(message).unwrap();
                            break;
                        }
                        _ => {
                            info!("Unknown command.");
                        }
                    }
                }
            }
            ipc_handle.close();
        });

        info!("Starting UI.");
        self.ui_handle.run::<UIModel>((
            Box::new(self.keyboard_handle),
            self.layout_definition,
            receiver,
        ));
    }
}
