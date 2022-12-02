use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crate::utils::Answer;

pub fn solve() -> Answer {
    let file = File::open("inputs/day1.txt");

    let reader = BufReader::new(file.unwrap());

    let mut cur = 0;
    let mut max = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }

    Answer::I32(max)
}
