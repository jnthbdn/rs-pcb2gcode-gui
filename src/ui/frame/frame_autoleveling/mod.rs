mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::ui::bool_to_str;

glib::wrapper! {
    pub struct FrameAutoleveling(ObjectSubclass<imp::FrameAutoleveling>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameAutoleveling {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    #[template_callback]
    pub fn enable_autolevel_toggled(&self, _: gtk::CheckButton) {
        self.imp().set_autolevel_enable(
            self.imp().enable_front.is_active() || self.imp().enable_back.is_active(),
        )
    }

    pub fn get_string_param(&self) -> Result<String, String> {
        let mut result = String::new();

        result += &format!(
            "--al-front={} ",
            bool_to_str(self.imp().enable_front.is_active())
        );

        result += &format!(
            "--al-back={} ",
            bool_to_str(self.imp().enable_back.is_active())
        );

        if self.imp().enable_front.is_active() || self.imp().enable_back.is_active() {
            result += &format!("--al-x={} ", self.imp().distance_probe_x.value_str(true));
            result += &format!("--al-y={} ", self.imp().distance_probe_y.value_str(true));
            result += &format!("--al-probefeed={} ", self.imp().feed.value_str(true));
            result += &format!("--al-prob-on={} ", self.imp().probe_on.text());
            result += &format!("--al-prob-off={} ", self.imp().probe_off.text());
            result += &format!(
                "--software={} ",
                self.imp()
                    .software
                    .selected_item()
                    .and_downcast_ref::<gtk::StringObject>()
                    .as_ref()
                    .unwrap()
                    .string()
            );

            if self.imp().software.selected() == 3 {
                result += &format!("--al-probecode={} ", self.imp().probe_code.text());
                result += &format!("--al-probevar={} ", self.imp().probe_variable.text());
                result += &format!("--al-setzzero={} ", self.imp().probe_set_zero.text());
            }
        }

        Ok(result)
    }
}
