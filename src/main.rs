//! # Yet Another Binero puzzle game

mod engine;
mod value;
mod difficulty;
mod gui;

//use fltk::{app::{App, AppScheme}, button::*, frame::Frame, image::PngImage, input::Input, menu::*, window::MenuWindow};
//use std::time::{Duration, Instant};
//use std::thread;
//use std::sync::mpsc;
//use std::path::Path;
use tr::tr_init;
use gui::Game;
//use dirs::config_dir;
//use std::fs::File;
//use std::io::Write;
//use std::rc::Rc;
//use std::cell::RefCell;

fn main() {
    tr_init!("locale");
    Game::new();
//    let mut frame = Frame::new(0, 40, 320, 40, "");
//    frame.set_align(Align::AlignRight);
//    let mut but = Button::new(160, 80, 100, 40, &tr!("Click me!"));
//    but.set_color(Color::Light2);
//    if let Ok(_config_file) = File::create(Path::new(&config_dir().unwrap()).join("yabinero")) {
//        //config_file.write(b"Test");
//    }
//    let now = Instant::now();
//    const WAITING_DURATION: Duration = Duration::from_millis(100);
//    let (tx, rx) = mpsc::channel();
//    thread::spawn(move || {
//        let mut pause = false;
//        loop {
//            thread::sleep(WAITING_DURATION);
//            if let Ok(p) = rx.try_recv() {
//                pause = p;
//            }
//            if !pause {
//                let duration = now.elapsed().as_secs();
//                let duration_as_str = format!("{:02}:{:02}:{:02}", duration / 3600, duration / 60, duration % 60);
//                frame.set_label(&duration_as_str);
//            }
//        }
//    });
//    but.set_callback(Box::new(move || {
//        app.set_scheme(AppScheme::Gtk);
//        tx.send(true).unwrap();
//    }));
}
