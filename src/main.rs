/*
    Talking clock
    Command line application which says the time.

    Copyright 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

mod config;
mod voice;
mod locale;
extern crate clioptions;
extern crate litepattern;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use clioptions::CliOptions;
use litepattern::LPattern;
use chrono::Local;
use config::Config;
use voice::Voice;
use locale::*;
use std::io::{Read, Write, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::process::exit;

fn display_version() {
    println!("talkingclock v.0.1.0.");
    exit(0);
}

fn display_error(program: &str, err: &str) {
    println!("Error: {}.\n", err);
    display_usage(program, -1);
}

fn display_usage(program: &str, code: i32) {
    println!("Talking clock");
    println!("Command line application which says the time.");
    println!("Copyright 2017 Sam Saint-Pettersen.");
    println!("\nReleased under the MIT License.");
    println!("\nUsage: {} [-h | -v | -t <hh:mm>][-q]", program);
    println!("\nOptions are:\n");
    println!("-t | --time: Input time in 24 hour notation.");
    println!("-q | --quiet: Do not talk, only display time (Quiet mode).");
    println!("-h | --help: Display this usage information and exit.");
    println!("-v | --version: Display version information and exit.");
    exit(code);
}

fn parse_unit(unit: &str) -> usize {
    let n = unit.parse::<usize>().ok();
    let unit = match n {
        Some(unit) => unit as usize,
        None => 0 as usize,
    };
    unit
}

fn throw_invalid_time(program: &str) {
    display_error(program, "Invalid format time");
}

fn write_config(conf: &str, config: &Config) {
    let mut w = File::create(conf).unwrap();
    let j = serde_json::to_string(&config).unwrap();
    let fo = format!("{:#}\n", j);
    let _ = w.write_all(fo.as_bytes());
}

fn load_config(conf: &str) -> Config {
    let mut cs = String::new();
    let mut file = File::open(&conf).unwrap();
    let _ = file.read_to_string(&mut cs);
    serde_json::from_str(&cs).unwrap()
}

fn load_locale(localestr: &str) -> Locale {
    let mut locale = Locale::new();
    if localestr != "en" {
        let file = File::open(&format!("locale/{}.json", localestr)).unwrap();
        for line in BufReader::new(file).lines() {
            locale.add_message(
            serde_json::from_str(&line.unwrap())
            .unwrap());
        }
    }
    locale
}

fn say_time(program: &str, timestr: String, conf: &Config, quiet: bool) {
    let voice = Voice::new(&conf.get_voice(), conf.is_24hr());
    let locale = load_locale(&conf.get_locale());
    let _24hr: bool = conf.is_24hr();
    let mut hrs24: usize = 0;
    let mut hrs: usize = 0;
    let mut mins: usize = 0;
    if timestr.is_empty() {
        let now = Local::now();
        hrs24 = parse_unit(&format!("{}", now.format("%H")));
        hrs = parse_unit(&format!("{}", now.format("%H"))) % 12;
        mins = parse_unit(&format!("{}", now.format("%M")));
    } else {
        let p = LPattern::new("%hh:%mm");
        let caps = p.apply_to(&timestr);
        if p.is_match(caps.clone(), &timestr) {
            hrs24 = parse_unit(&caps[0][0..2]);
            hrs = parse_unit(&caps[0][0..2]) % 12;
            mins = parse_unit(&caps[1][0..2]);
        } else {
            throw_invalid_time(program);
        }
        if hrs24 > 23 || mins > 59 {
            throw_invalid_time(program);
        }
    }
    let mut spoken_time: Vec<&str> = vec!["It's"];
    let period: Vec<&str> = vec!["am", "pm", ""];
    let periodp: Vec<&str> = vec!["eh em", "pee em"];
    let sunits: Vec<&str> = vec!["", "one", "two", "three", "four", "five", 
    "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen",
    "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "zero"];
    let stens: Vec<&str> = vec!["twenty", "thirty", "forty", "fifty", "oh"];
    let mut am_pm = period[0];
    let mut am_pmp = periodp[0];
    if !_24hr && hrs24 > 11 {
        am_pm = period[1];
        am_pmp = periodp[1];
    } else if _24hr {
        am_pm = period[2];
        am_pmp = period[2];
        hrs = hrs24;
    }
    if !_24hr && hrs == 0 { hrs = 12; }
    if _24hr && hrs < 10 { 
        if hrs == 0 {
            spoken_time.push(sunits[20]);
        }
        spoken_time.push(sunits[20]);
    }
    if _24hr && hrs >= 20 {
        spoken_time.push(stens[0]);
        if hrs > 20 {
            spoken_time.push(sunits[hrs - 20]);
        }
    } else {
        spoken_time.push(sunits[hrs]);
    }
    if _24hr && mins == 0 {
        spoken_time.push(sunits[20]);
        spoken_time.push(sunits[20]);
    }
    if mins >= 20 && mins < 30 {
        spoken_time.push(stens[0]);
        if mins > 20 {
            spoken_time.push(sunits[mins - 20]);
        }
    } else if mins >= 30 && mins < 40 {
        spoken_time.push(stens[1]);
        if mins > 30 {
            spoken_time.push(sunits[mins - 30]);
        }
    } else if mins >= 40 && mins < 50 {
        spoken_time.push(stens[2]);
        if mins > 40 {
            spoken_time.push(sunits[mins - 40]);
        }
    } else if mins >= 50 {
        spoken_time.push(stens[3]);
        if mins > 50 {
            spoken_time.push(sunits[mins - 50]);
        }
    } else if mins < 10 {
        if mins > 0 {
            spoken_time.push(stens[4]);
        }
        spoken_time.push(sunits[mins]);
    } else {
        spoken_time.push(sunits[mins]);
    }
    let time = format!("{} {}", spoken_time.join(" "), am_pmp);
    let loctime = localize(&format!("{} {}", spoken_time.join(" "), am_pm), &locale);
    println!("{}", loctime);
    if !quiet {
        if voice.is_synth() {
            voice.speak_time_synth(&phoneticize(&time, &locale));
        } else {
            voice.speak_time(hrs, mins, am_pm);
        }
    }
    exit(0);
}

fn main() {
    let cli = CliOptions::new("talkingclock");
    let program = cli.get_program();
    let mut timestr = String::new();
    let mut quiet = false;
    // ------------------------------
    let conf = "config.json";
    let voice = "scottish";
    let locale = "en";
    let _24hr = false;
    // ------------------------------
    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-t" | "--time" => timestr = cli.next_argument(i),
                "-q" | "--quiet" => quiet = true,
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                _ => continue,
            }
        }
    }
    let mut config = Config::new(voice, locale, _24hr);
    if !Path::new(&conf).exists() {
        write_config(&conf, &config);
    }
    config = load_config(&conf);
    say_time(&program, timestr, &config, quiet);
}
