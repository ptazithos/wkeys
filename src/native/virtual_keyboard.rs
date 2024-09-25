use wayland_client::{protocol::wl_keyboard::KeyState, Connection, EventQueue};

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

    pub fn key_press(&mut self, key: evdev::Key) {
        if let Some(keyboard) = &self.session_state.keyboard {
            keyboard.key(0, key.code().into(), KeyState::Pressed.into());
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }

    pub fn key_release(&mut self, key: evdev::Key) {
        if let Some(keyboard) = &self.session_state.keyboard {
            keyboard.key(0, key.code().into(), KeyState::Released.into());
            self.event_queue.roundtrip(&mut self.session_state).unwrap();
        }
    }
}
