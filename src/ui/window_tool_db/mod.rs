mod imp;
use gtk::{glib, prelude::IsA};

glib::wrapper! {
    pub struct WindowToolDB(ObjectSubclass<imp::WindowToolDB>)
        @extends gtk::Widget, gtk::Window;
}

#[gtk::template_callbacks]
impl WindowToolDB {
    pub fn new<P: IsA<gtk::Widget>>(parent_widget: &P) -> Self {
        glib::Object::builder()
            .property("transient-for", parent_widget)
            .build()
    }

    // #[template_callback]
    // fn add_to_counter(&self, button: &gtk::Button) {
    //     button.set_label("plop");
    // }
}
