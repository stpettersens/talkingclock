/*
    Talking clock
    Command line application which says the time.

    Copyright 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

use std::process::Command;
use std::thread;

fn play_sound(voice: &str, word: &str) {
    if cfg!(target_os = "windows") {
        Command::new("sounder.exe")
        .arg(&format!("voice/{}/{}.wav", voice, word))
        .spawn()
        .expect("sounder failed to start");
    } else {
        Command::new("ffplay")
        .args(&["-autoexit", "-nodisp", "-loglevel", "8", 
        &format!("{}/voice/{}.wav", voice, word)])
        .spawn()
        .expect("ffplay failed to start");
    }
    thread::sleep_ms(1000);
}

#[derive(Debug, Deserialize)]
pub struct Voice {
    voice: String,
}

impl Voice {
    pub fn speak_time(&self, hrs: usize, mins: usize, am_pm: &str) {
        let mut i = 1;
        for m in vec!["its", 
        &format!("{}", hrs), &format!("{}", mins), am_pm] {
            let mut w = m;
            let mut u = String::new();
            if i == 3 {
                if mins == 0 { i += 1; continue; }
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
            play_sound(&self.voice, &w);
            if !u.is_empty() {
                play_sound(&self.voice, &u);
            }
            i += 1;
        }
    }
}
