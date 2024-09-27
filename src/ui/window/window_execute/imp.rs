#![allow(unreachable_code)]

use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
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

        let write_buffer: Arc<Mutex<VecDeque<(bool, String)>>> =
            Arc::new(Mutex::new(VecDeque::new()));

        self.process_running.set(true);

        let mut child = child.unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let thread_buffer = write_buffer.clone();
        let thread_process_running = self.process_running.clone();
        thread::spawn(move || {
            log::debug!("Start stdout thread");
            let lines = BufReader::new(stdout).lines();

            for line in lines {
                if !thread_process_running.load(Ordering::Relaxed) {
                    break;
                }
                match line {
                    Ok(line) => {
                        thread_buffer.lock().unwrap().push_back((false, line));
                    }
                    Err(_) => (),
                };
            }

            log::debug!("End stdout thread");
        });

        let thread_process_running = self.process_running.clone();
        let thread_buffer = write_buffer.clone();
        thread::spawn(move || {
            log::debug!("Start stderr thread");
            let lines = BufReader::new(stderr).lines();

            for line in lines {
                if !thread_process_running.load(Ordering::Relaxed) {
                    break;
                }
                match line {
                    Ok(line) => {
                        thread_buffer.lock().unwrap().push_back((true, line));
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

            std::thread::sleep(std::time::Duration::from_secs(1));
            thread_process_running.store(false, Ordering::Relaxed);
            log::debug!("End watcher thread");
        });

        let thread_process_running = self.process_running.clone();
        let thread_buffer = write_buffer.clone();
        let thread_win = self.obj().clone();
        glib::source::idle_add(move || {
            if !thread_process_running.load(Ordering::Relaxed) {
                return glib::ControlFlow::Break;
            }

            let mut buffer = thread_buffer.lock().unwrap();

            while buffer.len() > 0 {
                let entry = buffer.pop_front().unwrap();

                if entry.0 {
                    thread_win.add_error_line(&entry.1);
                    log::error!("PCB2GCODE: {}", entry.1);
                } else {
                    thread_win.add_line(&entry.1);
                    log::info!("PCB2GCODE: {}", entry.1);
                }
            }

            drop(buffer);

            glib::ControlFlow::Continue
        });
    }

    pub fn add_error_line(&self, line: &str) {
        self.textview.buffer().insert_with_tags_by_name(
            &mut self.textview.buffer().end_iter(),
            &format!("[ERR] {line}\n"),
            &["error"],
        );
        self.scroll_down();
    }

    pub fn add_line(&self, line: &str) {
        self.textview
            .buffer()
            .insert(&mut self.textview.buffer().end_iter(), &format!("{line}\n"));
        self.scroll_down();
    }

    pub fn scroll_down(&self) {
        let buffer = self.textview.buffer();
        let mark = buffer.create_mark(None, &buffer.end_iter(), false);
        self.textview.scroll_mark_onscreen(&mark);
        buffer.delete_mark(&mark);
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
