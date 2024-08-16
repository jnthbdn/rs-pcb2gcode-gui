fn main() {
    glib_build_tools::compile_resources(
        &["src/ui/resources"],
        "src/ui/resources/resources.gresource.xml",
        "rs-pcb2gcode-gui.gresource",
    );
}
