// Guitar Chord Indicator
// by Asphalt, 2024.11.02

use std::env;
use std::str::FromStr;

mod print;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 6 {
        std::process::exit(-1);
    }

    let frets  = get_frets(args);
    let midi_pos = get_midi_pos(frets);

    print::print_midi(midi_pos);
}

fn get_frets(args: Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for s in args {
        if s == "x" {
            result.push(-1);
            continue;
        }

        let convert = i32::from_str(&s);

        let mut i: i32 = 0;
        match convert.is_ok() {
            true => i = convert.unwrap(),
            false => parse_error(),
        }

        if !(0..24).contains(&i) {
            parse_error();
        }

        result.push(i);
    }

    result
}

fn parse_error() {
    println!("parse error!");
    std::process::exit(-1);
}

fn get_midi_pos(frets: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![40, 45, 50, 55, 59, 64];

    for i in 0..6 {
        let j = frets[i];

        if j == -1 { result[i] = -1; }
        else { result[i] += j };
    }

    result
}
