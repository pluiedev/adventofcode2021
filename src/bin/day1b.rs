use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input1.txt").split_whitespace();
    
    dbg!(input
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(ta, tb)| ta < tb)
        .count());
}

