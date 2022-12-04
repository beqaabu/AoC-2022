use crate::utils::answer::AnsPair;
use crate::utils::Answer;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn solve() -> AnsPair {
    let file = File::open("inputs/day4.txt");

    let reader = BufReader::new(file.unwrap());

    let mut ans = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let (a, b) = line.split_once(",").unwrap();
        let (s1, e1) = a.split_once("-").unwrap();
        let (s2, e2) = b.split_once("-").unwrap();

        let s1 = s1.parse::<i32>().unwrap();
        let e1 = e1.parse::<i32>().unwrap();
        let s2 = s2.parse::<i32>().unwrap();
        let e2 = e2.parse::<i32>().unwrap();

        match check_intersection(s1, e1, s2, e2) {
            true => ans += 1,
            false => continue,
        }
        // if (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2) {
        //     ans += 1;
        // }
    }

    (Answer::I32(ans), Answer::I32(ans))
}

fn check_intersection(s1: i32, e1: i32, s2: i32, e2: i32) -> bool {
    s1 >= s2 && s1 <= e2 || e1 >= s2 && e1 <= e2 || s2 >= s1 && s2 <= e1 || e2 >= s1 && e2 <= e1
}
