use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::utils::answer::AnsPair;
use crate::utils::Answer;

pub fn solve() -> AnsPair {
    let file = File::open("inputs/day3.txt");

    let reader = BufReader::new(file.unwrap());

    let mut ans = 0;

    // for line in reader.lines() {
    //     let mut items_1 = HashMap::new();
    //     let mut items_2 = HashMap::new();
    //     let line = line.unwrap();
    //
    //     let (a, b) = line.split_at(line.len()/2);
    //
    //     for c in a.chars() {
    //         items_1.insert(c,1);
    //     }
    //
    //     for c in b.chars() {
    //         if items_1.contains_key(&c) {
    //             match c.is_lowercase() {
    //                 true => ans += c as i32 - 96,
    //                 false => ans += c as i32 - 38,
    //             }
    //             break
    //         } else {
    //             items_2.insert(c,1);
    //         }
    //     }
    // }

    let mut sack: Vec<String> = vec![];

    for line in reader.lines() {
        sack.push(line.unwrap());
    }

    let mut i = 0;
    while i < sack.len() - 2 {
        let c = find_common_elements(&sack[i], &sack[i + 1], &sack[i + 2]);
        match c.is_lowercase() {
            true => ans += c as i32 - 96,
            false => ans += c as i32 - 38,
        }
        i += 3;
    }

    (Answer::I32(ans), Answer::I32(ans))
}

fn find_common_elements(s1: &str, s2: &str, s3: &str) -> char {
    let set1: HashSet<char> = s1.chars().collect();
    let set2: HashSet<char> = s2.chars().collect();
    let set3: HashSet<char> = s3.chars().collect();

    let common_elements: HashSet<char> = set1.intersection(&set2).cloned().collect();
    let common_elements: HashSet<char> = common_elements.intersection(&set3).cloned().collect();

    common_elements.into_iter().collect::<Vec<char>>()[0]
}
