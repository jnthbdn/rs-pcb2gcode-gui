#![allow(unreachable_code)]

use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(gtk::CompositeTemplate, glib::Properties, Default)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window/window_execute.ui")]
#[properties(wrapper_type=super::WindowExecute)]
pub struct WindowExecute {
    #[template_child]
    pub textview: TemplateChild<gtk::TextView>,

    process_running: Arc<AtomicBool>,
    write_lock: Arc<AtomicBool>,
}

impl WindowExecute {
    pub fn run(&self, params: Vec<String>) {
        if self.process_running.load(Ordering::Relaxed) {
            log::warn!("Process already running...");
            return;
        }

        self.add_line("Start pcb2gcode");

        let child = Command::new("pcb2gcode")
            .args(params)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        if child.is_err() {
            let err = child.err().unwrap();
            self.textview.buffer().set_text(&format!(
                "ERROR : Failed to spawn process.\nError desciption: {}",
                err,
            ));
            log::error!("Failed to spaw process. {}", err);
            return;
        }

        self.process_running.set(true);
        self.write_lock.set(false);

        let mut child = child.unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let thread_write_lock = self.write_lock.clone();
        let thread_process_running = self.process_running.clone();
        let thread_win = self.obj().clone();
        thread::spawn(move || {
            log::debug!("Start stdout thread");
            let lines = BufReader::new(stdout).lines();

            for line in lines {
                if !thread_process_running.load(Ordering::Relaxed) {
                    break;
                }
                match line {
                    Ok(line) => {
                        while thread_write_lock.load(Ordering::Relaxed) {}
                        thread_write_lock.set(true);
                        log::info!("PCB2GCODE: {}", line);
                        thread_win.add_line(&line);
                        thread_write_lock.set(false);
                    }
                    Err(_) => (),
                };
            }

            log::debug!("End stdout thread");
        });

        let thread_write_lock = self.write_lock.clone();
        let thread_process_running = self.process_running.clone();
        let thread_win = self.obj().clone();
        thread::spawn(move || {
            log::debug!("Start stderr thread");
            let lines = BufReader::new(stderr).lines();

            for line in lines {
                if !thread_process_running.load(Ordering::Relaxed) {
                    break;
                }
                match line {
                    Ok(line) => {
                        while thread_write_lock.load(Ordering::Relaxed) {}
                        thread_write_lock.set(true);
                        log::error!("PCB2GCODE: {}", line);
                        thread_win.imp().add_error_line(&line);
                        thread_write_lock.set(false);
                    }
                    Err(_) => (),
                };
            }

            log::debug!("End stderr thread");
        });

        let thread_process_running = self.process_running.clone();
        thread::spawn(move || {
            log::debug!("Start watcher thread");

            match child.wait() {
                Ok(code) => log::info!("pcb2gcode thread exited with code {}", code),
                Err(e) => log::error!("pcb2gcode error \"{}\"", e),
            };

            thread_process_running.store(false, Ordering::Relaxed);
            log::debug!("End watcher thread");
        });
    }

    pub fn add_error_line(&self, line: &str) {
        self.textview.buffer().insert_with_tags_by_name(
            &mut self.textview.buffer().end_iter(),
            &format!("[ERR] {line}\n"),
            &["error"],
        );
    }

    pub fn add_line(&self, line: &str) {
        self.textview
            .buffer()
            .insert(&mut self.textview.buffer().end_iter(), &format!("{line}\n"));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WindowExecute {
    const NAME: &'static str = "WindowExecute";
    type Type = super::WindowExecute;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for WindowExecute {
    fn constructed(&self) {
        self.parent_constructed();

        self.process_running.store(false, Ordering::Relaxed);

        let buffer = &self.textview.buffer();

        buffer.create_tag(Some("error"), &[("foreground", &"#AA0000".to_value())]);
    }
}
impl WidgetImpl for WindowExecute {}
impl WindowImpl for WindowExecute {}
impl ApplicationWindowImpl for WindowExecute {}
