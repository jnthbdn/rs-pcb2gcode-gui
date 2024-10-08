pub mod frame {
    pub mod frame_autoleveling;
    pub mod frame_common;
    pub mod frame_drill;
    pub mod frame_input_output;
    pub mod frame_mill;
    pub mod frame_outline;
    pub mod frame_tool_settings;
    pub mod frame_tree_tools;
}

pub mod object {
    pub mod tree_tool_row;

    pub mod browse_file_object;
    pub mod db_label_object;
    pub mod entry_object;
    pub mod info_tooltip_object;
    pub mod select_tool_object;
    pub mod spin_button_object;
    pub mod textview_object;
}

pub mod window {
    pub mod window_command;
    pub mod window_execute;
    pub mod window_main;
    pub mod window_tool_db;
}

pub static READONLY_CSS_CLASS: &str = "readonly";

fn bool_to_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

fn show_alert_dialog(
    message: &str,
    detail: &str,
    parent: &impl gtk::glib::object::IsA<gtk::Window>,
) {
    gtk::AlertDialog::builder()
        .message(message)
        .detail(detail)
        .modal(true)
        .build()
        .show(Some(parent));
}
