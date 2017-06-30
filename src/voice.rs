/*
    Talking clock
    Command line application which says the time.

    Copyright 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

use std::fs::File;
use std::io::BufReader;
use rodio::{Source, Decoder, get_default_endpoint, play_raw};

pub struct Voice {

}

impl Voice {
    pub fn new() -> Voice {
        Voice {}
    }
    pub fn speak_time(&self, hrs: usize, mins: usize) {
        let endpoint = get_default_endpoint().unwrap();
        let file = File::open("its.wav").unwrap();
        let source = 
        Decoder::new(BufReader::new(file)).unwrap();
        play_raw(&endpoint, source.convert_samples());
    }
}
