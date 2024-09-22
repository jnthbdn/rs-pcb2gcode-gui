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
}

impl WindowExecute {
    pub fn run(&self, params: String) {
        if self.process_running.load(Ordering::Relaxed) {
            log::warn!("Process already running...");
            return;
        }

        self.add_line("Start pcb2gcode");

        let child = Command::new("pcb2gcode")
            .args(params.split(" ").collect::<Vec<&str>>())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        if child.is_err() {
            self.textview.buffer().set_text(&format!(
                "ERROR : Failed to spawn process.\nError desciption: {}",
                child.err().unwrap(),
            ));
            return;
        }

        self.process_running.set(true);

        let mut child = child.unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let thread_process_running = self.process_running.clone();
        let thread_win = self.obj().clone();
        thread::spawn(move || {
            log::info!("Start stdout thread");
            let lines = BufReader::new(stdout).lines();

            for line in lines {
                if !thread_process_running.load(Ordering::Relaxed) {
                    break;
                }
                match line {
                    Ok(line) => {
                        log::info!("STDOUT: {}", line);
                        thread_win.add_error_line(&line);
                    }
                    Err(_) => (),
                };
            }

            log::info!("End stdout thread");
        });

        let thread_process_running = self.process_running.clone();
        let thread_win = self.obj().clone();
        thread::spawn(move || {
            log::info!("Start stderr thread");
            let lines = BufReader::new(stderr).lines();

            for line in lines {
                if !thread_process_running.load(Ordering::Relaxed) {
                    break;
                }
                match line {
                    Ok(line) => {
                        log::info!("STDERR: {}", line);
                        thread_win.imp().add_error_line(&line);
                    }
                    Err(_) => (),
                };
            }

            log::info!("End stderr thread");
        });

        let thread_process_running = self.process_running.clone();
        thread::spawn(move || {
            log::info!("Start watcher thread");

            match child.wait() {
                Ok(code) => log::info!("pcb2gcode thread exited with code {}", code),
                Err(e) => log::warn!("pcb2gcode error \"{}\"", e),
            };

            thread_process_running.store(false, Ordering::Relaxed);
            log::info!("End watcher thread");
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
