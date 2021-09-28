use once_cell;
use gtk4::{
    glib::subclass::prelude::{ObjectSubclass, ObjectImpl},
    glib::{
        self,
        prelude::*,
        subclass::prelude::*
    }
};
use std::cell::{RefCell};

mod imp {
    use super::*;

    pub struct PersonObject {
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PersonObject {
        const NAME: &'static str = "PersonObject";
        type Type = super::wrp::PersonObject;
        type ParentType = glib::Object;
        type Interfaces = ();

        fn new() -> Self {
            Self {
                name: RefCell::default(),
            }
        }
    }

    impl ObjectImpl for PersonObject {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpec::new_string(
                        "name",
                        "Name",
                        "Name of this object",
                        None,
                        glib::ParamFlags::READWRITE,
                    )
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                "name" => {
                    let name = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.name.replace(name);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "name" => self.name.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}

mod wrp {
    use super::*;
    glib::wrapper! {
        pub struct PersonObject(ObjectSubclass<imp::PersonObject>);
    }

    impl PersonObject {
        pub fn new(name: String) -> Self {
            glib::Object::new(&[("name", &name)]).unwrap()
        }
    }
}

pub struct PersonObject {}
impl PersonObject {
    pub fn new(name: String) -> glib::Object {
        wrp::PersonObject::new(name).upcast::<glib::Object>()
    }

    pub fn static_type() -> glib::Type {
        wrp::PersonObject::static_type()
    }
}