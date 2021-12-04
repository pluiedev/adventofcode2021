#![feature(array_chunks)]
use adventofcode2021::prelude::*;

fn main() {
    let mut input = include_str!("../inputs/input4.txt").lines();

    let numbers = input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect_vec();

    let mut bingoes = input
        .group_by(|l| l.is_empty())
        .into_iter()
        .skip(1)
        .step_by(2)
        .map(|(_, bingo)| {
            let grid = bingo.fold(vec![], |mut v, x| {
                v.extend(
                    x.split_ascii_whitespace()
                        .map(|x| Some(x.parse::<u8>().unwrap())),
                );
                v
            });
            Bingo { grid }
        })
        .collect_vec();

    'top: for &n in &numbers {
        dbg!(n);
        for b in bingoes.iter_mut() {
            let res = b.update(n);
            if let Some(num) = res {
                let summed = b
                    .grid
                    .iter()
                    .filter_map(|cell| cell.map(u32::from))
                    .reduce(|acc, v| acc + v)
                    .unwrap();
                dbg!(num as u32 * summed);
                break 'top;
            }
        }
    }
}

#[derive(Debug)]
struct Bingo {
    grid: Vec<Option<u8>>,
}

impl Bingo {
    fn update(&mut self, n: u8) -> Option<u8> {
        let pos = self.grid.iter_mut().position(|val| {
            if let Some(v) = val {
                if *v == n {
                    *val = None;
                    return true;
                }
            }
            false
        })?;

        let column = pos % 5;
        // row
        let row_start = pos - column;
        let row_end = row_start + 5;
        // perhaps confusingly, `None` indicates that the cell has been marked.
        if self.grid[row_start..row_end].iter().all(Option::is_none) {
            return Some(n);
        }
        // column
        if self
            .grid
            .iter()
            .skip(column)
            .step_by(5)
            .all(Option::is_none)
        {
            return Some(n);
        }
        None
    }
}
