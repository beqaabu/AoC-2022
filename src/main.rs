mod days;
mod utils;

use utils::answer::Answer;
use days::*;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("Usage: cargo run <day>");
    }

    let day = args[1].parse::<u8>().unwrap();

    let ans = solver(day);

    println!("{}", ans());
}

fn solver(day: u8) -> fn() -> Answer {
    match day {
        1 => day1::solve,
        _ => todo!(),
    }
}