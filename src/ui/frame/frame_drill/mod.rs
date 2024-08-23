mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct FrameDrill(ObjectSubclass<imp::FrameDrill>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameDrill {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
