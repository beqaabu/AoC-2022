use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use crate::utils::answer::AnsPair;
use crate::utils::Answer;

pub fn solve() -> AnsPair {
    let file = File::open("inputs/day3.txt");

    let reader = BufReader::new(file.unwrap());

    let mut ans = 0;

    for line in reader.lines() {
        let mut items_1 = HashMap::new();
        let mut items_2 = HashMap::new();
        let line = line.unwrap();

        let (a, b) = line.split_at(line.len()/2);

        for c in a.chars() {
            items_1.insert(c,1);
        }

        for c in b.chars() {
            if items_1.contains_key(&c) {
                match c.is_lowercase() {
                    true => ans += c as i32 - 96,
                    false => ans += c as i32 - 38,
                }
                break
            } else {
                items_2.insert(c,1);
            }
        }
    }
    (Answer::I32(ans), Answer::U32(0))
}