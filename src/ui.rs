use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use tracing::info;

use crate::service::KeyboardHandle;

pub struct UIModel {
    keyboard_handle: Box<dyn KeyboardHandle>,
}

#[derive(Debug)]
pub enum UIMessage {
    Press,
    Release,
}

#[relm4::component(pub)]
impl SimpleComponent for UIModel {
    type Init = Box<dyn KeyboardHandle>;

    type Input = UIMessage;
    type Output = ();

    view! {
        gtk::Window {
            init_layer_shell: (),
            set_layer: Layer::Overlay,

            set_anchor: (Edge::Left, true),
            set_anchor: (Edge::Right, true),
            set_anchor: (Edge::Top, false),
            set_anchor: (Edge::Bottom, true),



            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Key Press",
                    connect_clicked => UIMessage::Press
                },

                gtk::Button::with_label("Key Release") {
                    connect_clicked => UIMessage::Release
                },
            }
        }
    }

    // Initialize the UI.
    fn init(
        handle: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = UIModel {
            keyboard_handle: handle,
        };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            UIMessage::Press => {
                info!("Press");
                self.keyboard_handle.key_press(evdev::Key::KEY_A);
            }
            UIMessage::Release => {
                info!("Release");
                self.keyboard_handle.key_release(evdev::Key::KEY_A);
            }
        }
    }
}
