mod ipc;
mod layout;
mod native;
mod service;
mod ui;

use layout::assets::LayoutAssets;
use native::VirtualKeyboard;
use service::AppService;
use single_instance::SingleInstance;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    let instance = SingleInstance::new("net.pithos.wkeys").unwrap();

    if instance.is_single() {
        let default_layout = LayoutAssets::get_default_60_percent_layout();

        let keyboard = VirtualKeyboard::new();

        // keyboard.key_press(evdev::Key::KEY_A);
        // keyboard.key_release(evdev::Key::KEY_A);

        let app_service = AppService::new(keyboard, default_layout);
        app_service.run();
    } else {
        info!("Another instance is already running.");
    }
}
