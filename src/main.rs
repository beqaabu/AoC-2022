mod days;
mod utils;

use crate::utils::answer::AnsPair;
use days::*;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("Usage: cargo run <day>");
    }

    let day = args[1].parse::<u8>().unwrap();

    let ans = solver(day);

    let (ans1, ans2) = ans();
    println!("first part: {ans1}, \nsecond part: {ans2}");
}

fn solver(day: u8) -> fn() -> AnsPair {
    match day {
        1 => day1::solve,
        2 => day2::solve,
        3 => day3::solve,
        4 => day4::solve,
        _ => todo!(),
    }
}
