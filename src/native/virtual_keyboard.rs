use wayland_client::{protocol::wl_keyboard::KeyState, Connection, EventQueue};

use crate::service::KeyboardHandle;

use super::session::SessionState;

pub struct VirtualKeyboard {
    session_state: SessionState,
    event_queue: EventQueue<SessionState>,
    modifiers: u32,
    locks: u32,
}

impl VirtualKeyboard {
    pub fn new() -> Self {
        let conn = Connection::connect_to_env().unwrap();
        let display = conn.display();

        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, ());

        let mut state = SessionState {
            keyboard_manager: None,
            keyboard: None,
            seat: None,
        };

        //bind seat and virtual keyboard manager
        event_queue.roundtrip(&mut state).unwrap();
        //create virtual keyboard by seat and manager
        event_queue.roundtrip(&mut state).unwrap();

        Self {
            session_state: state,
            event_queue: event_queue,
            modifiers: 0,
            locks: 0,
        }
    }
}

impl KeyboardHandle for VirtualKeyboard {
    fn key_press(&mut self, key: evdev::Key) {
        if let Some(keyboard) = &self.session_state.keyboard {
            keyboard.key(0, key.code().into(), KeyState::Pressed.into());
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    fn key_release(&mut self, key: evdev::Key) {
        if let Some(keyboard) = &self.session_state.keyboard {
            keyboard.key(0, key.code().into(), KeyState::Released.into());
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    fn append_mod(&mut self, key: evdev::Key) {
        let mod_code = Self::map_mod_key(key);
        self.modifiers |= mod_code;

        self.update_state();
    }

    fn remove_mod(&mut self, key: evdev::Key) {
        let mod_code = Self::map_mod_key(key);
        self.modifiers &= !mod_code;

        self.update_state();
    }

    fn append_lock(&mut self, key: evdev::Key) {
        let lock_code = Self::map_lock_key(key);
        self.locks |= lock_code;

        self.update_state();
    }

    fn remove_lock(&mut self, key: evdev::Key) {
        let lock_code = Self::map_lock_key(key);
        self.locks &= !lock_code;

        self.update_state();
    }
}

impl VirtualKeyboard {
    fn update_state(&mut self) {
        if let Some(keyboard) = &self.session_state.keyboard {
            keyboard.modifiers(self.modifiers, 0, self.locks, 0);
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    fn map_mod_key(key: evdev::Key) -> u32 {
        match key {
            evdev::Key::KEY_LEFTCTRL | evdev::Key::KEY_RIGHTCTRL => 4,
            evdev::Key::KEY_LEFTMETA | evdev::Key::KEY_RIGHTMETA => 4,
            evdev::Key::KEY_LEFTSHIFT | evdev::Key::KEY_RIGHTSHIFT => 1,
            evdev::Key::KEY_LEFTALT | evdev::Key::KEY_RIGHTALT => 8,
            _ => 0,
        }
    }

    fn map_lock_key(key: evdev::Key) -> u32 {
        match key {
            evdev::Key::KEY_CAPSLOCK => 2,
            evdev::Key::KEY_NUMLOCK => 256,
            evdev::Key::KEY_SCROLLLOCK => 32768,
            _ => 0,
        }
    }
}
