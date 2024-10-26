use wayland_client::{protocol::wl_keyboard::KeyState, Connection, EventQueue};

use crate::service::KeyboardHandle;

use super::session::SessionState;

pub struct VirtualKeyboard {
    session_state: SessionState,
    event_queue: EventQueue<SessionState>,
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

    fn set_mod(&mut self, key: evdev::Key) {
        if let Some(keyboard) = &self.session_state.keyboard {
            let mod_code = {
                match key {
                    evdev::Key::KEY_LEFTCTRL
                    | evdev::Key::KEY_LEFTMETA
                    | evdev::Key::KEY_RIGHTMETA
                    | evdev::Key::KEY_RIGHTCTRL => 4,

                    evdev::Key::KEY_LEFTSHIFT | evdev::Key::KEY_RIGHTSHIFT => 1,
                    evdev::Key::KEY_LEFTALT | evdev::Key::KEY_RIGHTALT => 8,

                    _ => 0,
                }
            };
            println!("mod_code: {}", mod_code);

            keyboard.modifiers(mod_code, 0, 0, 0);
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    fn remove_mod(&mut self) {
        if let Some(keyboard) = &self.session_state.keyboard {
            keyboard.modifiers(0, 0, 0, 0);
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }
}
