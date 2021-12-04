use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input1.txt").split_terminator('\n');
    
    dbg!(input
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(ta, tb)| ta < tb)
        .count());
}

