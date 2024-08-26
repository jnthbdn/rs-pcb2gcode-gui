mod imp;

use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct FrameInputOutput(ObjectSubclass<imp::FrameInputOutput>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameInputOutput {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn get_string_param(&self) -> Result<String, String> {
        let mut result = String::new();

        if !self.imp().front_file.get_path().is_empty() {
            result += &format!("--front=\"{}\" ", self.imp().front_file.get_path());
        }

        if self.imp().back_file.get_path().is_empty() {
            return Err("Back file is required !".to_string());
        } else {
            result += &format!("--back=\"{}\" ", self.imp().back_file.get_path());
        }

        if !self.imp().drill_file.get_path().is_empty() {
            result += &format!("--drill=\"{}\" ", self.imp().drill_file.get_path());
        }

        if !self.imp().outline_file.get_path().is_empty() {
            result += &format!("--outline=\"{}\" ", self.imp().outline_file.get_path());
        }

        if !self.imp().preamble_text_file.get_path().is_empty() {
            result += &format!(
                "--preamble-text=\"{}\" ",
                self.imp().preamble_text_file.get_path()
            );
        }

        if !self.imp().preamble_file.get_path().is_empty() {
            result += &format!("--preamble=\"{}\" ", self.imp().preamble_file.get_path());
        }

        if !self.imp().postamble_file.get_path().is_empty() {
            result += &format!("--postamble=\"{}\" ", self.imp().postamble_file.get_path());
        }

        if self.imp().output_folder.get_path().is_empty() {
            return Err("Output folder is required !".to_string());
        } else {
            result += &format!("--output-dir=\"{}\" ", self.imp().output_folder.get_path());
        }

        Ok(result)
    }
}
