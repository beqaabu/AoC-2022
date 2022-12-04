use crate::utils::answer::AnsPair;
use crate::utils::Answer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn solve() -> AnsPair {
    let file = File::open("inputs/day1.txt");

    let reader = BufReader::new(file.unwrap());

    let mut cur = 0;
    let mut max = 0;
    let mut calories = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            calories.push(cur);
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }

    calories.sort();
    let ans =
        calories[calories.len() - 1] + calories[calories.len() - 2] + calories[calories.len() - 3];
    (Answer::I32(max), Answer::I32(ans))
}
