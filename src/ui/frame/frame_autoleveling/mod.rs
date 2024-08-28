mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{settings::settings_frame_autolevel::SettingsFrameAutolevel, ui::bool_to_str};

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

    pub fn load_frame_settings(&self, settings: &SettingsFrameAutolevel) {
        let self_imp = self.imp();

        self_imp.enable_back.set_active(settings.is_enable_back());
        self_imp.enable_front.set_active(settings.is_enable_front());
        self_imp.software.set_selected(settings.software());
        self_imp
            .distance_probe_x
            .init_value(settings.distance_probe_x());
        self_imp
            .distance_probe_y
            .init_value(settings.distance_probe_y());
        self_imp.feed.init_value(settings.feed());
        self_imp.probe_on.set_text(settings.probe_on());
        self_imp.probe_off.set_text(settings.probe_off());
        self_imp.probe_code.set_text(settings.probe_code());
        self_imp.probe_variable.set_text(settings.probe_variable());
        self_imp.probe_set_zero.set_text(settings.probe_set_zero());
    }

    pub fn save_frame_settings(&self, settings: &mut SettingsFrameAutolevel) {
        let self_imp = self.imp();

        settings.set_is_enable_back(self_imp.enable_back.is_active());
        settings.set_is_enable_front(self_imp.enable_front.is_active());
        settings.set_software(self_imp.software.selected());
        settings.set_distance_probe_x(self_imp.distance_probe_x.value());
        settings.set_distance_probe_y(self_imp.distance_probe_y.value());
        settings.set_feed(self_imp.feed.value());
        settings.set_probe_on(self_imp.probe_on.text().into());
        settings.set_probe_off(self_imp.probe_off.text().into());
        settings.set_probe_code(self_imp.probe_code.text().into());
        settings.set_probe_variable(self_imp.probe_variable.text().into());
        settings.set_probe_set_zero(self_imp.probe_set_zero.text().into());
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
