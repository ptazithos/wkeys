use std::thread;

use gdk4::{
    prelude::{ObjectExt},
};
use gtk::prelude::{ApplicationExt, BoxExt, GtkWindowExt, ToggleButtonExt, WidgetExt};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use tracing::info;

use crate::{
    layout::parse::{KeyType, LayoutDefinition},
    service::{host::KeyboardHandle, IPCHandle},
    ui::main_view::gtk::Label,
    ProgramArgs,
};

use super::components::ButtonEX;

pub struct UIModel {
    keyboard_handle: Box<dyn KeyboardHandle>,
}

#[derive(Debug)]
pub enum UIMessage {
    ButtonPress(u16),
    ButtonRelease(u16),
    ModPress(u16),
    ModRelease(u16),
    LockPress(u16),
    LockRelease(u16),
    AppQuit,
}

impl SimpleComponent for UIModel {
    type Init = (
        Box<dyn KeyboardHandle>,
        Box<dyn IPCHandle + Send>,
        LayoutDefinition,
        ProgramArgs,
    );

    type Input = UIMessage;
    type Output = ();
    type Root = gtk::Window;
    type Widgets = ();

    fn init_root() -> Self::Root {
        // Create a window with a height of 1/3 of the smallest monitor.
        gtk::Window::builder().build()
    }

    // Initialize the UI.
    fn init(
        handle: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Create a thread to listen for the close command.
        let message_sender = sender.clone();
        let ipc_handle = handle.1;
        thread::spawn(move || loop {
            if let Ok(command) = String::from_utf8(ipc_handle.read()) {
                match command.as_str() {
                    "close" => {
                        info!("Received close command.");
                        message_sender.input(UIMessage::AppQuit);
                        break;
                    }
                    _ => {}
                }
            }
        });

        window.init_layer_shell();
        window.set_namespace(Some("wkeys"));
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::None);

        let anchors = [
            (Edge::Left, false),
            (Edge::Right, false),
            (Edge::Top, false),
            (Edge::Bottom, true),
        ];

        for (anchor, state) in anchors {
            window.set_anchor(anchor, state);
        }

        let model = UIModel {
            keyboard_handle: handle.0,
        };

        // window.emit_enable_debugging(true);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        window.set_child(Some(&container));
        //container.set_margin_all(5);
        container.set_align(gtk::Align::Center);
        container.set_expand(true);

        let keyboard_definition = handle.2;
        let geometry_unit = handle.3.height;

        keyboard_definition.layout.iter().for_each(|row| {
            let row_container = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();

            row.iter().for_each(|key| {
                let scan_code = key.scan_code;
                let width =
                    (key.width.unwrap_or(1.0) * f32::from(geometry_unit as u16)).round() as i32;

                match key.key_type() {
                    KeyType::Mod => {
                        let toggle = gtk::ToggleButton::builder()
                            .label(format!(
                                "{} {}",
                                key.bottom_legend.clone().unwrap_or_default(),
                                key.top_legend.clone().unwrap_or_default()
                            ))
                            .width_request(width)
                            .height_request(geometry_unit)
                            .build();

                        let button_sender = sender.clone();
                        toggle.connect_toggled(move |btn| {
                            if btn.is_active() {
                                button_sender.input(UIMessage::ModPress(scan_code));
                            } else {
                                button_sender.input(UIMessage::ModRelease(scan_code));
                            }
                        });

                        row_container.append(&toggle);
                    }
                    KeyType::Lock => {
                        let toggle = gtk::ToggleButton::builder()
                            .label(format!(
                                "{} {}",
                                key.bottom_legend.clone().unwrap_or_default(),
                                key.top_legend.clone().unwrap_or_default()
                            ))
                            .width_request(width)
                            .height_request(geometry_unit)
                            .build();

                        let button_sender = sender.clone();
                        toggle.connect_toggled(move |btn| {
                            if btn.is_active() {
                                button_sender.input(UIMessage::LockPress(scan_code));
                            } else {
                                button_sender.input(UIMessage::LockRelease(scan_code));
                            }
                        });

                        row_container.append(&toggle);
                    }
                    KeyType::Normal => {

                        if scan_code == 0 {
                            let label = Label::default();
                            label.set_width_request(width);
                            row_container.append(&label);
                        }
                        else {
                            let button = ButtonEX::default();

                            button.set_primary_content(key.top_legend.clone().unwrap_or_default());
                            button.set_secondary_content(key.bottom_legend.clone().unwrap_or_default());

                            button.set_width_request(width);
                            button.set_height_request(geometry_unit);
                            
                            let press_sender = sender.clone();
                            button.connect("pressed", true, move |_| {
                                press_sender.input(UIMessage::ButtonPress(scan_code));
                                None
                            });

                            let release_sender = sender.clone();
                            button.connect("released", true, move |_| {
                                release_sender.input(UIMessage::ButtonRelease(scan_code));
                                None
                            });

                            row_container.append(&button);
                        }
                    }
                }
            });

            container.append(&row_container);
        });

        let widgets = ();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            UIMessage::ButtonPress(scan_code) => {
                self.keyboard_handle
                    .key_press(evdev::KeyCode::new(scan_code));
            }
            UIMessage::ButtonRelease(scan_code) => {
                self.keyboard_handle
                    .key_release(evdev::KeyCode::new(scan_code));
            }
            UIMessage::ModPress(scan_code) => {
                self.keyboard_handle
                    .append_mod(evdev::KeyCode::new(scan_code));
            }
            UIMessage::ModRelease(scan_code) => {
                self.keyboard_handle
                    .remove_mod(evdev::KeyCode::new(scan_code));
            }
            UIMessage::LockPress(scan_code) => {
                self.keyboard_handle
                    .append_lock(evdev::KeyCode::new(scan_code));
            }
            UIMessage::LockRelease(scan_code) => {
                self.keyboard_handle
                    .remove_lock(evdev::KeyCode::new(scan_code));
            }
            UIMessage::AppQuit => {
                self.keyboard_handle.destroy();
                relm4::main_application().quit();
            }
        }
    }

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
