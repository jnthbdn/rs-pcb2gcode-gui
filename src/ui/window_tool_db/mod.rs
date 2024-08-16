mod imp;
use gtk::glib;

glib::wrapper! {
    pub struct WindowToolDB(ObjectSubclass<imp::WindowToolDB>)
        @extends gtk::Widget, gtk::Window;
}

#[gtk::template_callbacks]
impl WindowToolDB {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    // #[template_callback]
    // fn add_to_counter(&self, button: &gtk::Button) {
    //     button.set_label("plop");
    // }
}
