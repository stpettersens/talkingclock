/*
    Talking clock
    Command line application which says the time.

    Copyright 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

use std::process::Command;

pub struct Voice {

}

impl Voice {
    pub fn new() -> Voice {
        Voice {}
    }
    pub fn speak_time(&self, hrs: usize, mins: usize) {
        if cfg!(target_os = "windows") {
            Command::new("sounder.exe")
            .arg("its.wav")
            .spawn()
            .expect("sounder failed to start");
        } else {
            Command::new("ffplay")
            .args(&["-autoexit", "-nodisp", "-loglevel", "8", "its.wav"])
            .spawn()
            .expect("ffplay failed to start");
        }
    }
}
