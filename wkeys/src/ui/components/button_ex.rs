use std::{borrow::BorrowMut, cell::RefCell, sync::OnceLock};

use gdk4::glib::{subclass::Signal, Properties};
use gtk::{glib, prelude::*, subclass::prelude::*};
use relm4::gtk;

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type = ButtonEX)]
pub struct ButtonInner {
    #[property(get, set)]
    primary_content: RefCell<Option<String>>,
    #[property(get, set)]
    secondary_content: RefCell<Option<String>>,
    #[property(get, set)]
    layout: RefCell<Option<gtk::Box>>,
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

        obj.connect_primary_content_notify(|obj| {
            obj.update_view();
        });

        obj.connect_secondary_content_notify(|obj| {
            obj.update_view();
        });

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

        let stylus = gtk::GestureStylus::new();

        let obj_up_cb = obj.clone();
        stylus.connect_up(move |stylus, _, _| {
            stylus.set_state(gtk::EventSequenceState::Claimed);
            obj_up_cb.emit_by_name::<()>("released", &[]);
        });
        
        let obj_down_cb = obj.clone();
        stylus.connect_down(move |stylus, _, _| {
            stylus.set_state(gtk::EventSequenceState::Claimed);
            obj_down_cb.emit_by_name::<()>("pressed", &[]);
        });

        obj.add_controller(stylus);
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

    fn dispose(&self) {
        if let Some(child) = self.layout.borrow_mut().take() {
            child.unparent();
        }
    }
}

impl WidgetImpl for ButtonInner {}

glib::wrapper! {
    pub struct ButtonEX(ObjectSubclass<ButtonInner>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ButtonEX {
    pub fn update_view(&self) {
        if let Some(child) = self.layout().borrow_mut().take() {
            child.unparent();
        }

        let primary_content = self.primary_content();
        let secondary_content = self.secondary_content();

        let new_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .valign(gtk::Align::Center)
            .build();

        if let Some(primary_content) = primary_content {
            if primary_content.len() > 0 {
                let primary_content = gtk::Label::new(Some(primary_content.as_str()));
                new_layout.append(&primary_content);
            }
        }

        if let Some(secondary_content) = secondary_content {
            if secondary_content.len() > 0 {
                let secondary_content = gtk::Label::new(Some(secondary_content.as_str()));
                new_layout.append(&secondary_content);
            }
        }

        new_layout.set_parent(&*self);
        self.set_layout(new_layout);
    }
}

impl Default for ButtonEX {
    fn default() -> Self {
        glib::Object::new()
    }
}
