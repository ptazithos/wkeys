use gdk4::{
    prelude::{DisplayExt, ListModelExtManual, MonitorExt},
    Monitor,
};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

use tracing::info;

use crate::{layout::parse::LayoutDefinition, service::KeyboardHandle};

pub struct UIModel {
    keyboard_handle: Box<dyn KeyboardHandle>,
}

#[derive(Debug)]
pub enum UIMessage {
    Press,
    Release,
}

impl SimpleComponent for UIModel {
    type Init = (Box<dyn KeyboardHandle>, LayoutDefinition);

    type Input = UIMessage;
    type Output = ();
    type Root = gtk::Window;
    type Widgets = ();

    fn init_root() -> Self::Root {
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

        // Create a window with a height of 1/3 of the smallest monitor.
        gtk::Window::builder()
            .height_request(screen_height / 3)
            .build()
    }

    // Initialize the UI.
    fn init(
        handle: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);

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

        window.emit_enable_debugging(true);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        window.set_child(Some(&container));
        container.set_margin_all(5);
        container.set_align(gtk::Align::Center);

        let keyboard_definition = handle.1;
        keyboard_definition.layout.iter().for_each(|row| {
            let row_container = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(5)
                .build();

            row.iter().for_each(|key| {
                let button = gtk::Button::builder()
                    .label(format!(
                        "{:?} {:?}",
                        key.bottom_legend.clone().unwrap_or_default(),
                        key.top_legend.clone().unwrap_or_default()
                    ))
                    .build();

                row_container.append(&button);
            });

            container.append(&row_container);
        });

        let widgets = ();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            UIMessage::Press => {
                info!("Press");
                self.keyboard_handle.key_press(evdev::Key::KEY_GRAVE);
            }
            UIMessage::Release => {
                info!("Release");
                self.keyboard_handle.key_release(evdev::Key::KEY_GRAVE);
            }
        }
    }

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
