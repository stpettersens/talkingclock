/*
    Talking clock
    Command line application which says the time.

    Copyright 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

use std::process::Command;
use std::{thread, time};

fn play_sound(voice: &str, word: &str) {
    if cfg!(target_os = "windows") {
        Command::new("sounder.exe")
        .arg(&format!(r"voice\{}\{}.wav", voice, word))
        .spawn()
        .expect("sounder failed to start");
    } else {
        Command::new("ffplay")
        .args(&["-autoexit", "-nodisp", "-loglevel", "8", 
        &format!("voice/{}/{}.wav", voice, word)])
        .spawn()
        .expect("ffplay failed to start");
    }
    thread::sleep(time::Duration::from_millis(1000));
}

pub struct Voice {
    voice: String,
    _24hr: bool,
}

impl Voice {
    pub fn new(voice: &str, _24hr: bool) -> Voice {
        Voice {
            voice: voice.to_owned(),
            _24hr: _24hr,
        }
    }
    pub fn is_synth(&self) -> bool {
        let mut synth = false;
        if &self.voice[0..self.voice.len() - 1] == "synth" {
            synth = true;
        }
        synth
    }
    fn set_synth_gender(&self) -> &str {
        let mut gender = "-f";
        if self.voice == "synthm" {
            gender = "-m";
        }
        &gender
    }
    pub fn speak_time_synth(&self, time: &str) {
        if cfg!(target_os = "windows") {
            Command::new("voice.exe")
            .arg(self.set_synth_gender())
            .arg(time)
            .spawn()
            .expect("voice failed to start");
        } else {
            Command::new("say")
            .arg(time)
            .spawn()
            .expect("say failed to start");
        }
    }
    pub fn speak_time(&self, hrs: usize, mins: usize, am_pm: &str) {
        let mut i = 1;
        for m in vec!["its", &format!("{}", hrs), &format!("{}", mins), am_pm] {
            let mut w = m;
            let mut ww = String::new();
            let mut www = String::new();
            let mut u = String::new();
            if self._24hr && hrs == 0 { 
                ww = format!("{}", 0);
                www = format!("{}", 0);
            }
            if self._24hr && hrs < 10 { 
                ww = format!("{}", 0);
                www = format!("{}", hrs);
            } else if self._24hr && hrs >= 20 {
                ww = format!("{}", 20);
                if hrs > 20 {
                    www = format!("{}", hrs - 20);
                }
            }
            if i == 3 {
                if !self._24hr && mins == 0 { i += 1; continue; }
                if self._24hr && mins == 0 {
                    w = "0";
                    u = format!("{}", 0);
                }
                if mins > 0 && mins < 10 {
                    w = "O";
                    u = format!("{}", mins);
                }
                if mins > 20 && mins < 30 {
                    w = "20";
                    u = format!("{}", mins % 20);
                }
                if mins > 30 && mins < 40 {
                    w = "30";
                    u = format!("{}", mins % 30);
                }
                if mins > 40 && mins < 50 {
                    w = "40";
                    u = format!("{}", mins % 40);
                }
                if mins > 50 {
                    w = "50";
                    u = format!("{}", mins % 50);
                }
            }
            if i == 2 && !ww.is_empty() {
                play_sound(&self.voice, &ww);
                if !www.is_empty() { play_sound(&self.voice, &www); }
            } else {
                play_sound(&self.voice, &w);
            }
            if !u.is_empty() {
                play_sound(&self.voice, &u);
            }
            i += 1;
        }
    }
}
