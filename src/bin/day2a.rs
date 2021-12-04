use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input2.txt").split_whitespace();

    let (depth, hposition) = input
        .tuples()
        .fold((0, 0), |(mut depth, mut hposition), (dir, amount)| {
            let amount = str::parse::<u32>(amount).unwrap();
            match dir {
                "forward" => hposition += amount,
                "up" => depth -= amount,
                "down" => depth += amount,
                _ => unreachable!()
            }
            (depth, hposition)
        });

    dbg!(depth, hposition);
    // code here
}
    
