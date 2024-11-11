use std::{sync::mpsc::Receiver, thread};

use gdk4::{
    prelude::{DisplayExt, ListModelExtManual, MonitorExt, ObjectExt},
    Monitor,
};
use gtk::prelude::{ApplicationExt, BoxExt, GtkWindowExt, ToggleButtonExt, WidgetExt};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

use crate::{
    layout::parse::{KeyType, LayoutDefinition},
    service::host::KeyboardHandle,
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
    type Init = (Box<dyn KeyboardHandle>, LayoutDefinition, Receiver<String>);

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
        thread::spawn(move || loop {
            if let Ok(command) = handle.2.recv() {
                if command == "close" {
                    message_sender.input(UIMessage::AppQuit);
                    break;
                }
            }
        });

        // Get the height of the smallest monitor.
        let screen_height = if let Some(display) = gdk4::Display::default() {
            let monitors = display.monitors();

            monitors
                .iter::<Monitor>()
                .map(|monitor| {
                    let monitor = monitor.unwrap();
                    monitor.geometry().height()
                })
                .min()
                .unwrap_or(1080)
        } else {
            1080
        };

        let window_height = screen_height / 4;

        window.set_height_request(window_height);

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::None);

        let anchors = [
            (Edge::Left, true),
            (Edge::Right, true),
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

        let keyboard_definition = handle.1;
        let geometry_unit = cal_geometry_unit(window_height, keyboard_definition.height);

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
            });

            container.append(&row_container);
        });

        let widgets = ();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            UIMessage::ButtonPress(scan_code) => {
                self.keyboard_handle.key_press(evdev::Key::new(scan_code));
            }
            UIMessage::ButtonRelease(scan_code) => {
                self.keyboard_handle.key_release(evdev::Key::new(scan_code));
            }
            UIMessage::ModPress(scan_code) => {
                self.keyboard_handle.append_mod(evdev::Key::new(scan_code));
            }
            UIMessage::ModRelease(scan_code) => {
                self.keyboard_handle.remove_mod(evdev::Key::new(scan_code));
            }
            UIMessage::LockPress(scan_code) => {
                self.keyboard_handle.append_lock(evdev::Key::new(scan_code));
            }
            UIMessage::LockRelease(scan_code) => {
                self.keyboard_handle.remove_lock(evdev::Key::new(scan_code));
            }
            UIMessage::AppQuit => {
                self.keyboard_handle.destroy();
                relm4::main_application().quit();
            }
        }
    }

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}

fn cal_geometry_unit(length: i32, count: i32) -> i32 {
    length / count
}
