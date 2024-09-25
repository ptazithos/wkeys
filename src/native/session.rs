use std::{ffi::CString, fs::File, io::Write, os::fd::AsFd, path::PathBuf};
use wayland_client::{
    protocol::{
        wl_keyboard::{self},
        wl_registry,
        wl_seat::{self, WlSeat},
    },
    Connection, Dispatch, Proxy, QueueHandle, WEnum,
};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::{self, ZwpVirtualKeyboardManagerV1},
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};
use xkbcommon::xkb;

pub struct SessionState {
    pub keyboard_manager: Option<ZwpVirtualKeyboardManagerV1>,
    pub keyboard: Option<ZwpVirtualKeyboardV1>,
    pub seat: Option<WlSeat>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for SessionState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<SessionState>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "zwp_virtual_keyboard_manager_v1" => {
                    let keyboard =
                        registry.bind::<ZwpVirtualKeyboardManagerV1, _, _>(name, version, &qh, ());

                    state.keyboard_manager = Some(keyboard);
                }
                "wl_seat" => {
                    let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
                    state.seat = Some(seat);
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1, ()> for SessionState {
    fn event(
        _state: &mut Self,
        _proxy: &zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
        _event: <zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSeat, ()> for SessionState {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<SessionState>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                if let Some(keyboard_manager) = &state.keyboard_manager {
                    let keyboard = keyboard_manager.create_virtual_keyboard(seat, qh, ());

                    let (file, len) = get_keymap_as_file();
                    keyboard.keymap(wl_keyboard::KeymapFormat::XkbV1.into(), file.as_fd(), len);
                    state.keyboard = Some(keyboard);
                }
            }
        }
    }
}

impl Dispatch<ZwpVirtualKeyboardV1, ()> for SessionState {
    fn event(
        _state: &mut Self,
        _proxy: &ZwpVirtualKeyboardV1,
        _event: <ZwpVirtualKeyboardV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

pub fn get_keymap_as_file() -> (File, u32) {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    let keymap = xkb::Keymap::new_from_names(
        &context,
        "",
        "",
        "us",
        "",
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .expect("xkbcommon keymap panicked!");
    let xkb_state = xkb::State::new(&keymap);
    let keymap = xkb_state
        .get_keymap()
        .get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
    let keymap = CString::new(keymap).expect("Keymap should not contain interior nul bytes");
    let keymap = keymap.as_bytes_with_nul();
    let dir = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let mut file = tempfile::tempfile_in(dir).expect("File could not be created!");
    file.write_all(keymap).unwrap();
    file.flush().unwrap();
    (file, keymap.len() as u32)
}
