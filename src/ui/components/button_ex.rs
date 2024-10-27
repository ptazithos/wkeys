use std::{cell::RefCell, sync::OnceLock};

use gdk4::glib::{subclass::Signal, Properties};
use gtk::{glib, prelude::*, subclass::prelude::*};
use relm4::gtk;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type = ButtonEX)]
pub struct ButtonInner {
    #[property(get, set)]
    content: RefCell<Option<String>>,
    child: RefCell<Option<gtk::Label>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ButtonInner {
    const NAME: &'static str = "ExButton";
    type Type = ButtonEX;
    type ParentType = gtk::Widget;

    fn class_init(class: &mut Self::Class) {
        class.set_layout_manager_type::<gtk::BinLayout>();

        class.set_css_name("button");

        class.set_accessible_role(gtk::AccessibleRole::Button);
    }
}

#[glib::derived_properties]
impl ObjectImpl for ButtonInner {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        *self.content.borrow_mut() = None;

        let child = gtk::Label::new(Some(obj.content().unwrap_or_default().as_str()));

        child.set_parent(&*obj);
        obj.bind_property("content", &child, "label")
            .sync_create()
            .build();
        *self.child.borrow_mut() = Some(child);

        let gesture = gtk::GestureClick::new();

        let obj_release_cb = obj.clone();
        gesture.connect_pressed(move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            obj_release_cb.emit_by_name::<()>("pressed", &[]);
        });

        let obj_press_cb = obj.clone();
        gesture.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            obj_press_cb.emit_by_name::<()>("released", &[]);
        });

        obj.add_controller(gesture);
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("pressed").build(),
                Signal::builder("released").build(),
            ]
        })
    }
}

impl WidgetImpl for ButtonInner {}

glib::wrapper! {
    pub struct ButtonEX(ObjectSubclass<ButtonInner>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ButtonEX {
    fn default() -> Self {
        glib::Object::new()
    }
}
