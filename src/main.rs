mod layout;
mod native;
mod service;
mod ui;

use tracing::info;

use layout::assets::LayoutAssets;
use native::VirtualKeyboard;
use service::AppService;

fn main() {
    tracing_subscriber::fmt::init();

    let default_layout = LayoutAssets::get_default_60_percent_layout();
    info!("Default layout: {:?}", default_layout);

    let keyboard = VirtualKeyboard::new();

    // keyboard.key_press(evdev::Key::KEY_A);   
    // keyboard.key_release(evdev::Key::KEY_A);

    let app_service = AppService::new(keyboard);
    app_service.run();
}
