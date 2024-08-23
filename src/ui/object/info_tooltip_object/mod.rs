mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt, template_callbacks};

glib::wrapper! {
    pub struct InfoToolTipObject(ObjectSubclass<imp::InfoToolTipObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable;
}

#[template_callbacks]
impl InfoToolTipObject {
    #[template_callback]
    fn button_clicked(&self, _: gtk::Button) {
        self.imp().popover.set_visible(true);
    }
}
