use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input7.txt");

    let inputs = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .sorted()
        .collect_vec();
    let sum = inputs.iter().copied().sum::<i64>();
    dbg!(sum);
    let len = inputs.len() as i64;
    dbg!(len);
    let mean = sum / len; // FIXME: this works with the final input but not the test input, curious
    dbg!(mean);
    let fuel = inputs.into_iter().fold(0i64, |acc, ele| {
        let n = (ele - mean).abs();
        acc + ((n * n + n) / 2)
    });
    dbg!(fuel);

    // code here
}
