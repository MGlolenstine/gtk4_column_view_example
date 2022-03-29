use gtk::glib::{self, Object, ParamFlags, ParamSpec, ParamSpecString, ToValue, Value, ParamSpecUInt};
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use std::cell::RefCell;

// Object holding the state
#[derive(Default)]
pub struct GItemObject {
    id: RefCell<u32>,
    name: RefCell<String>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GItemObject {
    const NAME: &'static str = "ItemObject";
    type Type = ItemObject;
    type ParentType = glib::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for GItemObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecUInt::new(
                    "id",
                    "id",
                    "id",
                    0,
                    u32::MAX,
                    0,
                    ParamFlags::READWRITE,
                ),
                ParamSpecString::new(
                    "name",
                    "name",
                    "name",
                    Some("<Empty>"),
                    ParamFlags::READWRITE,
                ),
                
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "id" => {
                let input = value
                    .get()
                    .expect("The value needs to be of type `u32`.");
                self.id.replace(input);
            }
            "name" => {
                let input = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.name.replace(input);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "id" => self.id.borrow().to_value(),
            "name" => self.name.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}

glib::wrapper! {
    pub struct ItemObject(ObjectSubclass<GItemObject>);
}

impl ItemObject {
    pub fn new(id: u32, name: &str) -> Self {
        Object::new(&[
            ("id", &id),
            ("name", &name),
        ])
        .expect("Could not create `ItemObject`.")
    }
}
