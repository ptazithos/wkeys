use tracing::info;
use wayland_client::{protocol::wl_keyboard::KeyState, Connection, EventQueue};

use crate::service::host::KeyboardHandle;

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
    fn key_press(&mut self, key: evdev::KeyCode) {
        if let Some(keyboard) = &self.session_state.keyboard {
            info!("Key Pressed: {:?}", key);
            keyboard.key(0, key.code().into(), KeyState::Pressed.into());
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    fn key_release(&mut self, key: evdev::KeyCode) {
        if let Some(keyboard) = &self.session_state.keyboard {
            info!("Key Released: {:?}", key);
            keyboard.key(0, key.code().into(), KeyState::Released.into());
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    fn append_mod(&mut self, key: evdev::KeyCode) {
        info!("Mod Appended: {:?}", key);
        let mod_code = Self::map_mod_key(key);
        self.modifiers |= mod_code;

        self.update_state();
    }

    fn remove_mod(&mut self, key: evdev::KeyCode) {
        info!("Mod Removed: {:?}", key);
        let mod_code = Self::map_mod_key(key);
        self.modifiers &= !mod_code;

        self.update_state();
    }

    fn append_lock(&mut self, key: evdev::KeyCode) {
        info!("Lock Appended: {:?}", key);
        let lock_code = Self::map_lock_key(key);
        self.locks |= lock_code;

        self.update_state();
    }

    fn remove_lock(&mut self, key: evdev::KeyCode) {
        info!("Lock Removed: {:?}", key);
        let lock_code = Self::map_lock_key(key);
        self.locks &= !lock_code;

        self.update_state();
    }

    fn destroy(&mut self) {
        if let Some(keyboard) = &self.session_state.keyboard {
            info!("Destroying Virtual Keyboard.");
            keyboard.destroy();
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }
}

impl VirtualKeyboard {
    fn update_state(&mut self) {
        if let Some(keyboard) = &self.session_state.keyboard {
            keyboard.modifiers(self.modifiers, 0, self.locks, 0);
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    fn map_mod_key(key: evdev::KeyCode) -> u32 {
        match key {
            evdev::KeyCode::KEY_LEFTCTRL | evdev::KeyCode::KEY_RIGHTCTRL => 4,
            evdev::KeyCode::KEY_LEFTMETA | evdev::KeyCode::KEY_RIGHTMETA => 64,
            evdev::KeyCode::KEY_LEFTSHIFT | evdev::KeyCode::KEY_RIGHTSHIFT => 1,
            evdev::KeyCode::KEY_LEFTALT | evdev::KeyCode::KEY_RIGHTALT => 8,
            _ => 0,
        }
    }

    fn map_lock_key(key: evdev::KeyCode) -> u32 {
        match key {
            evdev::KeyCode::KEY_CAPSLOCK => 2,
            evdev::KeyCode::KEY_NUMLOCK => 256,
            evdev::KeyCode::KEY_SCROLLLOCK => 32768,
            _ => 0,
        }
    }
}
