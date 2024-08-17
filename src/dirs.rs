use std::{fs::create_dir_all, path::PathBuf};

use directories::ProjectDirs;

pub fn get_config_dir() -> PathBuf {
    let proj_dir = ProjectDirs::from("com.github", "jnthbdn", "rs-pcb2gcode-gui")
        .expect("Unable to find project directory... Unsupported OS ?");

    let path = proj_dir.data_dir();
    create_dir_all(path).expect(&format!(
        "Failed to create configuration dir ('{}')",
        path.display()
    ));

    path.to_path_buf()
}

pub fn get_config_path_to(file: &str) -> PathBuf {
    let mut config_dir = get_config_dir();
    config_dir.push(file);

    config_dir
}
