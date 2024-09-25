use native::VirtualKeyboard;

mod native;

fn main() {
    tracing_subscriber::fmt::init();

    let mut keyboard = VirtualKeyboard::new();

    keyboard.key_press(evdev::Key::KEY_A);
    keyboard.key_release(evdev::Key::KEY_A);
}
