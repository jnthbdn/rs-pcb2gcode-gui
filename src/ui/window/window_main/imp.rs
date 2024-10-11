#![allow(unreachable_code)]

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ffi::OsString,
    fs,
    iter::Map,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use gtk::{gdk, glib, prelude::*, subclass::prelude::*};

use crate::{
    database::database::Database,
    settings::Settings,
    ui::{
        frame::{
            frame_autoleveling::FrameAutoleveling, frame_common::FrameCommon,
            frame_drill::FrameDrill, frame_input_output::FrameInputOutput, frame_mill::FrameMill,
            frame_outline::FrameOutline,
        },
        window::window_tool_db::WindowToolDB,
    },
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window/window_main.ui")]
#[properties(wrapper_type=super::WindowMain)]
pub struct WindowMain {
    pub database: Arc<Mutex<Database>>,

    pub win_tool_db: RefCell<Option<WindowToolDB>>,

    pub settings: RefCell<Settings>,

    #[template_child]
    pub frame_input_output: TemplateChild<FrameInputOutput>,

    #[template_child]
    pub frame_common: TemplateChild<FrameCommon>,

    #[template_child]
    pub frame_mill: TemplateChild<FrameMill>,

    #[template_child]
    pub frame_drill: TemplateChild<FrameDrill>,

    #[template_child]
    pub frame_outline: TemplateChild<FrameOutline>,

    #[template_child]
    pub frame_autolevel: TemplateChild<FrameAutoleveling>,

    #[template_child]
    pub picture_preview: TemplateChild<gtk::Picture>,

    #[template_child]
    pub select_image: TemplateChild<gtk::DropDown>,

    svg_height: Cell<i32>,

    svg_files: RefCell<HashMap<OsString, PathBuf>>,
}

impl WindowMain {
    pub fn search_and_load_svg(&self) {
        let search_folder = self.frame_input_output.imp().output_folder.get_path();
        let search_folder = Path::new(&search_folder);

        let mut map = self.svg_files.borrow_mut();
        map.clear();

        if search_folder.exists() {
            let svg_files = match fs::read_dir(search_folder) {
                Ok(read_dir) => read_dir
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .filter_map(|path| {
                        if path.extension().map_or(false, |ext| ext == "svg") {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<PathBuf>>(),

                Err(e) => {
                    log::error!("Unable to read directory '{:?}': {e}", search_folder);
                    Vec::new()
                }
            };

            for svg in svg_files {
                map.insert(svg.file_name().unwrap().into(), svg);
            }
        }

        drop(map);
        self.update_image_dropdown();
    }

    fn update_image_dropdown(&self) {
        let string_list = gtk::StringList::default();

        let map = self.svg_files.borrow();

        for (name, _) in map.iter() {
            match name.to_str() {
                Some(s) => string_list.append(s),
                None => log::warn!("Unable to contert '{:?}' to &str", name),
            }
        }

        self.select_image.set_model(Some(&string_list));
    }

    fn show_selected_image(&self) {
        if self.select_image.selected_item().is_none() {
            self.picture_preview.set_paintable(None::<&gdk::Paintable>);
            return;
        }

        let image_name: OsString = self
            .select_image
            .selected_item()
            .and_downcast_ref::<gtk::StringObject>()
            .unwrap()
            .string()
            .into();

        let map = self.svg_files.borrow();

        if map.contains_key::<OsString>(&image_name) {
            match gdk::gdk_pixbuf::Pixbuf::from_file_at_size(
                map.get(&image_name).unwrap(),
                -1,
                self.svg_height.get(),
            ) {
                Ok(pixbuf) => self
                    .picture_preview
                    .set_paintable(Some(&gdk::Texture::for_pixbuf(&pixbuf))),
                Err(e) => {
                    log::error!("Failed to load svg file '{:?}'. Error: {e}", image_name);
                    self.picture_preview.set_paintable(None::<&gdk::Paintable>);
                }
            }
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WindowMain {
    const NAME: &'static str = "WindowMain";
    type Type = super::WindowMain;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        FrameInputOutput::ensure_type();
        FrameCommon::ensure_type();
        FrameMill::ensure_type();
        FrameDrill::ensure_type();
        FrameOutline::ensure_type();
        FrameAutoleveling::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for WindowMain {
    fn constructed(&self) {
        self.parent_constructed();

        self.frame_mill.set_database(self.database.clone());
        self.frame_drill.set_database(self.database.clone());
        self.frame_outline.set_database(self.database.clone());
        self.obj().output_unit_change(true);

        self.svg_height.set(0);

        for monitor in &self.obj().display().monitors() {
            match monitor {
                Ok(monitor) => {
                    let monitor = monitor.downcast_ref::<gdk::Monitor>().unwrap();
                    if monitor.geometry().height() > self.svg_height.get() {
                        self.svg_height.set(monitor.geometry().height());
                    }
                }
                Err(_) => (),
            };
        }

        let obj = self.obj().clone();
        self.select_image
            .connect_notify_local(Some("selected"), move |_, _| {
                obj.imp().show_selected_image()
            });
    }
}

impl WidgetImpl for WindowMain {}
impl WindowImpl for WindowMain {
    fn close_request(&self) -> glib::Propagation {
        self.obj().save_window_settings();

        glib::Propagation::Proceed
    }
}
impl ApplicationWindowImpl for WindowMain {}
