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

use fltk::{app::*, button::*, frame::*, window::*};
use std::fs::File;
use std::io::BufReader;
use rodio::Source;

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(Box::new(|| {
        let device = rodio::default_output_device().unwrap();
        let file = File::open("sounds/success.ogg").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
    }));
    app.run().unwrap();
}
