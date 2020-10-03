#![windows_subsystem = "windows"]

//! # Yet Another Binero puzzle game

mod engine;
mod enums;
mod gui;

use tr::tr_init;
use gui::Game;

fn main() {
    tr_init!("locale");
    let mut game = Game::new();
    game.show_window();
    game.add_menu_entries();
    game.run_app();
}
