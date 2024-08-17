// mod custom_object;
mod database;
mod dirs;
mod tools;
mod ui;
// mod window_main;
// mod window_tool_db;

use gtk::{gdk::Display, gio, glib, prelude::*, Application, CssProvider};
use ui::{custom_object, window_main, window_tool_db};

const APP_ID: &str = "com.github.jnthbdn.rs-pcb2gcode-gui";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("rs-pcb2gcode-gui.gresource")
        .expect("[main] Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(|app| {
        let win_main = window_main::WindowMain::new(app);
        win_main.present();
        win_main.open_tool_db(&gtk::Button::new());
    });

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/github/jnthbdn/rs-pcb2gcode-gui/css/window_main.css");
    provider.load_from_resource("/com/github/jnthbdn/rs-pcb2gcode-gui/css/window_tool_db.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
