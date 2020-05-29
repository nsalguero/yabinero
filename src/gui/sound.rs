//! # Sound
//!
//! `sound` contains the functions to play the sounds of the game

use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use rodio::Source;

/// The two possible types of sound
#[derive(Debug)]
pub enum Sound {
    Success,
    Error,
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printable = format!("{:?}", self);
        printable.make_ascii_lowercase();
        write!(f, "{}.ogg", printable)
    }
}

impl Sound {
    /// Plays a sound
    pub fn play(&self) {
        let device = rodio::default_output_device().unwrap();
        let file = File::open(Path::new("sounds").join(format!("{}", *self))).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
    }
}
