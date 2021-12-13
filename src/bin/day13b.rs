#![feature(btree_drain_filter)]
#![feature(int_abs_diff)]
use std::collections::BTreeSet;

use adventofcode2021::prelude::*;
use regex::Regex;
use termion::{cursor, clear};
fn main() {
    let input = include_str!("../inputs/input13.txt").lines();

    let mut set = BTreeSet::<(u16, u16)>::new();
    let regex = Regex::new(r"^fold along (?P<axis>x|y)=(?P<num>\d+)$").unwrap();

    for l in input {
        if l.is_empty() {
            continue;
        }

        if l.starts_with("fold along ") {
            // println!("start fold: {:?}", set);
            for caps in regex.captures_iter(l) {
                let num = caps["num"].parse::<u16>().unwrap();
                match &caps["axis"] {
                    "x" => {
                        let off = set
                            .drain_filter(|(x, _)| *x > num)
                            .map(|(x, y)| (x.abs_diff(2 * num), y))
                            .collect_vec();
                        // println!("folded: {:?}", off);  
                        set.extend(off);
                        // println!("after fold: {:?} ({})", set, set.len());
                    }
                    "y" => {
                        let off = set
                            .drain_filter(|(_, y)| *y > num)
                            .map(|(x, y)| (x, y.abs_diff(2 * num)))
                            .collect_vec();
                        // println!("folded: {:?}", off);  
                        set.extend(off);
                        // println!("after fold: {:?} ({})", set, set.len());
                    }
                    _ => {}
                }
            }
        } else {
            let (x, y) = l.split_once(',').unwrap();
            let x = x.parse::<u16>().unwrap();
            let y = y.parse::<u16>().unwrap();
            set.insert((x, y));
        }
    }

    // print
    println!("{}", clear::All);
    for (x, y) in set {
        println!("{}#", cursor::Goto(x + 1, y + 1));
    }
}