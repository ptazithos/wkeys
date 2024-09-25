mod native;
mod service;

use native::VirtualKeyboard;
use service::AppService;

fn main() {
    tracing_subscriber::fmt::init();

    let keyboard = VirtualKeyboard::new();

    // keyboard.key_press(evdev::Key::KEY_A);
    // keyboard.key_release(evdev::Key::KEY_A);

    let mut app_service = AppService::new(keyboard);
    app_service.run();
}
