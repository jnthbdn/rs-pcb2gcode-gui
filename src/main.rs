// mod custom_object;
mod database;
mod dirs;
mod tools;
mod ui;
// mod window_main;
// mod window_tool_db;

use gtk::{gdk::Display, gio, glib, prelude::*, Application, CssProvider};
use ui::{window::window_main, window::window_tool_db};

const APP_ID: &str = "com.github.jnthbdn.rs-pcb2gcode-gui";

fn main() -> glib::ExitCode {
    flexi_logger::Logger::try_with_env_or_str("info")
        .expect("Failed init logger")
        .format(flexi_logger::opt_format)
        .start()
        .expect("Fail start logger");

    gio::resources_register_include!("rs-pcb2gcode-gui.gresource")
        .expect("[main] Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(|app| {
        let win_main = window_main::WindowMain::new(app);
        win_main.present();
    });

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/github/jnthbdn/rs-pcb2gcode-gui/css/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
