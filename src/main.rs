//! # Yet Another Binero puzzle game

//mod engine;
//mod value;
//mod difficulty;
//mod gui;
//
//use engine::Binero;
//use difficulty::Difficulty;
//
//fn main() {
//    let mut game = Binero::new(12, Difficulty::Beginner);
//    println!("New game: {}", game);
//    println!("Can be solved: {}", game.try_to_solve());
//    println!("New game: {}", game);
//}

use fltk::{app::{App, AppScheme}, button::*, frame::Frame, image::PngImage, menu::*, window::Window};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use std::path::Path;

mod gui;

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust").center_screen();
    if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
        wind.set_icon(&icon);
    }
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    let now = Instant::now();
    const WAITING_DURATION: Duration = Duration::from_millis(100);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut pause = false;
        loop {
            thread::sleep(WAITING_DURATION);
            if let Ok(p) = rx.try_recv() {
                pause = p;
            }
            if !pause {
                let duration = now.elapsed().as_secs();
                let duration_as_str = format!("{:02}:{:02}:{:02}", duration / 3600, duration / 60, duration % 60);
                frame.set_label(&duration_as_str);
            }
        }
    });
    but.set_callback(Box::new(move || {
        app.set_scheme(AppScheme::Gtk);
        tx.send(true).unwrap();
        gui::sound::play(gui::sound::Sound::Error);
    }));
    app.run().unwrap();
}
