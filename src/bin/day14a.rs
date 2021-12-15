#![feature(array_windows)]
use std::collections::BTreeSet;

use adventofcode2021::prelude::*;
use itertools::MinMaxResult;
use regex::Regex;

fn main() {
    let input = include_str!("../inputs/input14.txt");

    let rules_regex = Regex::new(r"(\w)(\w) -> (\w)").unwrap();
    let (template, rules) = input.split_once("\n\n").unwrap();
    let mut polymer = template.chars().collect_vec();
    println!("{:?}", polymer);

    let rules = rules_regex
        .captures_iter(rules)
        .map(|c| Rule {
            first: c[1].chars().next().unwrap(),
            second: c[2].chars().next().unwrap(),
            target: c[3].chars().next().unwrap(),
        })
        .collect_vec();

    for i in 0..10 {
        let mut new_chars = vec![];
        for rule in &rules {
            // println!("{:?}", rule);
            for (pos, (a, b)) in polymer.iter().tuple_windows().enumerate() {
                if *a == rule.first && *b == rule.second {
                    // println!("{}, {}", pos, rule.target);
                    new_chars.push((pos + 1, rule.target));
                }
            }
        }
        new_chars.sort_by(|a, b| b.cmp(a));

        // println!("{:?}", new_chars);
        for (pos, nc) in new_chars {
            polymer.insert(pos, nc);
        }
        println!("i={}", i);
    }
    let cnts = polymer.into_iter().counts();
    dbg!(&cnts);
    match cnts.values().minmax() {
        MinMaxResult::MinMax(&min, &max) => dbg!(max-min),
        _ => unreachable!("got less than two elements")
    };
}

#[derive(Debug)]
struct Rule {
    first: char,
    second: char,
    target: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NewChar(usize, char);

impl PartialOrd for NewChar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for NewChar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}