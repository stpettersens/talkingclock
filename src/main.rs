/*
    Talking clock
    Command line application which says the time.

    Copyright 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

extern crate clioptions;
extern crate litepattern;
extern crate chrono;
use clioptions::CliOptions;
use litepattern::LPattern;
use chrono::Local;
use std::process::exit;

fn display_version() {
    println!("talkingclock v.0.1.0.");
    exit(0);
}

fn display_usage(program: &str) {
    println!("Talking clock");
    println!("Command line application which says the time.");
    println!("Copyright 2017 Sam Saint-Pettersen.");
    println!("\nReleased under the MIT License.");
    println!("\nUsage: {} [-h | -v | -t <hh:mm>]", program);
    println!("\nOptions are:\n");
    println!("-t | --time: Input time in 24 hour notation.");
    println!("-h | --help: Display this usage information and exit.");
    println!("-v | --version: DIsplay version information and exit.");
    exit(0);
}

fn parse_unit(unit: &str) -> usize {
    let n = unit.parse::<usize>().ok();
    let unit = match n {
        Some(unit) => unit as usize,
        None => 0 as usize,
    };
    unit
}

fn throw_invalid_time() {
    println!("Error: Invalid format time!");
    exit(-1);
}

fn say_time(timestr: String) {
    let mut hrs: usize = 0;
    let mut mins: usize = 0;
    if timestr.is_empty() {
        let now = Local::now();
        hrs = parse_unit(&format!("{}", now.format("%H")));
        mins = parse_unit(&format!("{}", now.format("%M")));
    } else {
        let p = LPattern::new("%hh:%mm");
        let caps = p.apply_to(&timestr);
        if p.is_match(caps.clone(), &timestr) {
            hrs = parse_unit(&caps[0][0..2]);
            mins = parse_unit(&caps[1][0..2]);
        } else {
            throw_invalid_time();
        }
        if hrs > 23 || mins > 59 {
            throw_invalid_time();
        }
    }
    let mut spoken_time: Vec<&str> = vec!["It's"];
    let period: Vec<&str> = vec!["am", "pm"];
    let sunits: Vec<&str> = vec!["", "one", "two", "three", "four", "five", 
    "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen",
    "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let stens: Vec<&str> = vec!["twenty", "thirty", "fourty", "fifty", "oh"];
    let mut am_pm = period[0];
    match hrs {
        12 => { am_pm = period[1]; },
        13 => { hrs = 1; am_pm = period[1]; },
        14 => { hrs = 2; am_pm = period[1]; },
        15 => { hrs = 3; am_pm = period[1]; },
        16 => { hrs = 4; am_pm = period[1]; },
        17 => { hrs = 5; am_pm = period[1]; },
        18 => { hrs = 6; am_pm = period[1]; },
        19 => { hrs = 7; am_pm = period[1]; },
        20 => { hrs = 8; am_pm = period[1]; },
        21 => { hrs = 9; am_pm = period[1]; },
        22 => { hrs = 10; am_pm = period[1]; },
        23 => { hrs = 11; am_pm = period[1]; },
        0 => { hrs = 12; am_pm = period[0]; },
        _ => {},
    }
    spoken_time.push(sunits[hrs]);
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
    println!("{} {}", spoken_time.join(" "), am_pm);
    exit(0);
}

fn main() {
    let cli = CliOptions::new("talkingclock");
    let program = cli.get_program();
    let mut timestr = String::new();
    if cli.get_num() > 0 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-t" | "--time" => timestr = cli.next_argument(i),
                "-h" | "--help" => display_usage(&program),
                "-v" | "--version" => display_version(),
                _ => continue,
            }
        }
    }
    say_time(timestr);
}
