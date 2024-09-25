pub trait KeyboardHandle {
    fn key_press(&mut self, key: evdev::Key);
    fn key_release(&mut self, key: evdev::Key);
}

pub struct AppService<M: KeyboardHandle> {
    pub keyboard_handle: M,
}

impl<M: KeyboardHandle> AppService<M> {
    pub fn new(keyboard_handle: M) -> Self {
        Self { keyboard_handle }
    }

    pub fn run(&mut self) {
        self.keyboard_handle.key_press(evdev::Key::KEY_A);
        self.keyboard_handle.key_release(evdev::Key::KEY_A);
    }
}
