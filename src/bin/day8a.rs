use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input8.txt").lines();

    let count = input.fold(0, |mut acc, s| {
        let (_, output) = s.split_once(" | ").unwrap();
        for digit in output.split_ascii_whitespace() {
            match digit.len() {
                2 | 3 | 4 | 7 => acc += 1,
                _ => {}
            }
        }
        acc
    });
    dbg!(count);
    // code here
}
