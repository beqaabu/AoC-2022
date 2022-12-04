use crate::utils::Answer;

use crate::utils::answer::AnsPair;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn solve() -> AnsPair {
    let file = File::open("inputs/day2.txt");

    let reader = BufReader::new(file.unwrap());

    let mut score1 = 0;
    let mut score2 = 0;

    let answers_for_1 = HashMap::from([
        (('A', 'X'), 4),
        (('A', 'Y'), 8),
        (('A', 'Z'), 3),
        (('B', 'X'), 1),
        (('B', 'Y'), 5),
        (('B', 'Z'), 9),
        (('C', 'X'), 7),
        (('C', 'Y'), 2),
        (('C', 'Z'), 6),
    ]);

    let answers_for_2 = HashMap::from([
        (('A', 'X'), 3),
        (('A', 'Y'), 4),
        (('A', 'Z'), 8),
        (('B', 'X'), 1),
        (('B', 'Y'), 5),
        (('B', 'Z'), 9),
        (('C', 'X'), 2),
        (('C', 'Y'), 6),
        (('C', 'Z'), 7),
    ]);

    for line in reader.lines() {
        let line = line.unwrap();
        let a = line.chars().next().unwrap();
        let b = line.chars().last().unwrap();

        score1 += answers_for_1.get(&(a, b)).unwrap();
        score2 += answers_for_2.get(&(a, b)).unwrap();
    }

    (Answer::I32(score1), Answer::I32(score2))
}
