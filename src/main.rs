#![windows_subsystem = "windows"]

//! # Yet Another Binero puzzle game

mod engine;
mod enums;
mod gui;

use std::{fs::File, io::Result, path::PathBuf};
use locale_config::Locale;
use gettext::Catalog;
use tr::set_translator;
use gui::Game;

fn main() {
    tr_init();
    let mut game = Game::new();
    game.show_window();
    game.add_menu_entries();
    game.run_app();
}

/// Initialises the translation
fn tr_init() {
    let locale = format!("{}", Locale::current());
    if let Ok(file) = open_mo_file(&locale) {
        init_catalog(file);
    } else {
        let loc: Vec<&str> = locale.split("-").collect();
        if let Ok(file) = open_mo_file(loc[0]) {
            init_catalog(file);
        }
    }
}

/// Opens a MO file and returns it
///
/// # Arguments
///
/// * `locale` - the name of a locale
fn open_mo_file(locale: &str) -> Result<File> {
    let mut loc_path = PathBuf::new();
    loc_path.push("locale");
    loc_path.push(locale);
    loc_path.push("LC_MESSAGES");
    loc_path.push(&format!("{}{}", env!("CARGO_PKG_NAME"), ".mo"));
    File::open(loc_path)
}

/// Initialises the catalog from a MO file
///
/// # Arguments
///
/// * `file` - a MO file
fn init_catalog(file: File) {
    let catalog = Catalog::parse(file).unwrap();
    set_translator!(catalog);
}
