use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input1.txt").split_terminator('\n');
    
    dbg!(input
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count());
}
