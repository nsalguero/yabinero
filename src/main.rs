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

use fltk::{app::{self, App, AppScheme}, button::*, frame::Frame, image::PngImage, menu::*, window::MenuWindow};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use tr::*;

//mod gui;

fn main() {
    tr_init!("locale");
    let app = App::default();
    let mut wind = MenuWindow::new(100, 100, 400, 300, "Hello from rust").center_screen();
    if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
        wind.set_icon(&icon);
    }
    wind.set_color(Color::Light2);
    let mut menu = MenuBar::new(0, 0, 400, 40, "");
    menu.set_color(Color::Light2);
    menu.set_selection_color(Color::Dark3);
    let mut frame = Frame::new(0, 40, 400, 160, "");
    let mut but = Button::new(160, 210, 80, 40, &tr!("Click me!"));
    but.set_color(Color::Light2);
    wind.end();
    wind.show();
    menu.add(&(tr!("Game") + "/" + &tr!("New") + "\t"), Shortcut::Ctrl + 'n', MenuFlag::MenuDivider, Box::new(|| {}));
    menu.add(&(tr!("Game") + "/" + &tr!("Quit") + "\t"), Shortcut::Ctrl + 'q', MenuFlag::Normal, Box::new(|| {std::process::exit(0)}));
    menu.add(&(tr!("Options") + "/" + &tr!("Sounds")), Shortcut::None, MenuFlag::Toggle, Box::new(|| {}));
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
    }));
    app.run().unwrap();
}
