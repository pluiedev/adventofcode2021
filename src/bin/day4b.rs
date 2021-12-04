#![feature(array_chunks)]
use std::collections::HashSet;

use adventofcode2021::prelude::*;

fn main() {
    let mut input = include_str!("../inputs/input4.txt").lines();

    let numbers = input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect_vec();

    let mut bingoes = vec![];
    for (_, bingo) in input.group_by(|l| l.is_empty()).into_iter().skip(1).step_by(2) {
        let grid = bingo.fold(vec![], |mut v, x| {
            dbg!(x);
            v.extend(
                x.split_ascii_whitespace()
                    .map(|x| (false, x.parse::<u8>().unwrap())),
            );
            v
        });
        println!();
        bingoes.push(Bingo { grid })
    }

    let count_bingoes = bingoes.len();
    let mut completed_bingoes = HashSet::new();
    'top: for &n in &numbers {
        dbg!(n);
        for (index, b) in bingoes.iter_mut().enumerate() {
            let res = dbg!(b.update(n));
            b.help();
            match res {
                Some(num) => {
                    let summed = b.grid.iter()
                        .filter(|(set, _)| !*set)
                        .map(|(_, v)| *v as u32)
                        .reduce(|acc, v| acc + v)
                        .unwrap_or(0);
                    
                    completed_bingoes.insert(index);
                    if completed_bingoes.len() == count_bingoes {
                        dbg!(num as u32 * summed);
                        break 'top;
                    }
                }
                None => {},
            }
        }
    }

    // dbg!(bingoes);
    // code here
}

#[derive(Debug)]
struct Bingo {
    grid: Vec<(bool, u8)>,
}


impl Bingo {
    fn update(&mut self, n: u8) -> Option<u8> {
        let pos = self.grid.iter_mut().position(|(set, val)| {
            let b = *val == n;
            if b {
                *set = b;
            }
            b
        })?;
        
        let column = pos % 5;
        // row
        let row_start = pos - column;
        let row_end = row_start + 5;
        if self.grid[row_start..row_end].iter().all(|(set, _)| *set) {
            return Some(n);
        }
        // column
        if self.grid.iter().skip(column).step_by(5).all(|(set, _)| *set) {
            return Some(n);
        }
        None
    }
    fn help(&self) {
        for a in self.grid.chunks_exact(5) {
            println!("{:?}", a);
        }
        println!()
    }
}
