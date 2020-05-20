//! # Yet Another Binero puzzle game

mod engine;
mod value;
mod difficulty;
mod gui;

use engine::Binero;
use difficulty::Difficulty;

//fn main() {
//    let mut game = Binero::new(12, Difficulty::Beginner);
//    println!("New game: {}", game);
//    println!("Can be solved: {}", game.try_to_solve());
//    println!("New game: {}", game);
//}

use fltk::{app::{App, AppScheme}, button::*, frame::Frame, image::PngImage, input::Input, menu::*, window::MenuWindow};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use tr::{tr, tr_init};
use dirs::config_dir;
use std::fs::File;
//use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let game = Binero::new(4, Difficulty::Beginner);
    tr_init!("locale");
    let app = App::default();
    let mut wind = MenuWindow::new(100, 100, 400, 520, "Hello from rust").center_screen();
    if let Ok(icon) = PngImage::load(&Path::new("icons").join("icon.png")) {
        wind.set_icon(&icon);
    }
    wind.set_color(Color::Light2);
    wind.make_resizable(false);
    let mut menu = MenuBar::new(0, 0, 400, 40, "");
    menu.set_color(Color::Light2);
    menu.set_selection_color(Color::Dark3);
    let mut frame = Frame::new(0, 40, 320, 40, "");
    frame.set_align(Align::AlignRight);
    let mut but = Button::new(160, 80, 100, 40, &tr!("Click me!"));
    but.set_color(Color::Light2);
    let mut boxes = Vec::new();
    for i in 0..4 {
        boxes.push(Vec::new());
        for j in 0..4 {
            let mut input = Input::new(j * 32, 120 + i * 32, 32, 32, "");
            if let Some(val) = game.get(i as u8, j as u8) {
                input.set_value(&format!(" {}", val));
                input.set_readonly(true);
                input.set_text_color(Color::Inactive);
                input.set_selection_color(Color::Dark1);
            } else {
                input.set_value(" ");
            }
            input.set_text_size(20);
            input.set_selection_color(Color::Dark3);
            boxes[i as usize].push(input);
        }
    }
    let boxes = Rc::new(RefCell::new(boxes));
    let game = Rc::new(RefCell::new(game));
    wind.end();
    wind.show();
    for i in 0..4 {
        for j in 0..4 {
            let cloned_boxes = Rc::clone(&boxes);
            let cloned_game = Rc::clone(&game);
            boxes.borrow_mut()[i as usize][j as usize].handle(Box::new(move |ev: Event| {
                match ev {
                    Event::KeyUp => {
                        let value = cloned_boxes.borrow()[i as usize][j as usize].value();
                        if let Ok(val) = value.trim().parse() {
                            if val != 0 && val != 1 {
                                cloned_boxes.borrow_mut()[i as usize][j as usize].undo();
                            } else {
                                let old_value = cloned_game.borrow().get(i, j);
                                if old_value != value::from_u8(val) {
                                    if cloned_game.borrow_mut().try_to_put(i, j, value::from_u8(val)) {
                                        cloned_boxes.borrow_mut()[i as usize][j as usize].set_value(&format!(" {}", value.trim()));
                                    } else {
                                        cloned_boxes.borrow_mut()[i as usize][j as usize].undo();
                                    }
                                }
                            }
                        }
                        true
                    },
                    _ => false,
                }
            }));
        }
    }
    menu.add(&(tr!("Game") + "/" + &tr!("New") + "\t"), Shortcut::Ctrl + 'n', MenuFlag::MenuDivider, Box::new(|| {}));
    menu.add(&(tr!("Game") + "/" + &tr!("Quit") + "\t"), Shortcut::Ctrl + 'q', MenuFlag::Normal, Box::new(|| {std::process::exit(0)}));
    menu.add(&(tr!("Options") + "/" + &tr!("Sounds")), Shortcut::None, MenuFlag::Toggle, Box::new(|| {}));
    if let Ok(_config_file) = File::create(Path::new(&config_dir().unwrap()).join("yabinero")) {
        //config_file.write(b"Test");
    }
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
