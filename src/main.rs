//! # Yet Another Binero puzzle game

mod engine;
mod size;
mod value;
mod difficulty;
mod gui;

use tr::tr_init;
use gui::Game;
//use dirs::config_dir;
//use std::fs::File;
//use std::io::Write;

fn main() {
    tr_init!("locale");
    let mut game = Game::new();
    game.show_window();
    game.add_menu_entries();
    game.run_app();
//    if let Ok(_config_file) = File::create(Path::new(&config_dir().unwrap()).join("yabinero")) {
//        //config_file.write(b"Test");
//    }
}
