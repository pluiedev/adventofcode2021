#![feature(drain_filter)]

use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input3.txt").split_terminator('\n');

    let mut bits = 0;
    let numbers = input
        .map(|x| {
            bits = x.len();
            u32::from_str_radix(x, 2).unwrap()
        })
        .collect_vec();

    let oxygen = count_bits_and_filter_till_theres_only_one_left(
        numbers.clone(),
        bits,
        |b, shift| !b.bit(shift),
        |a, b| a >= b,
    );
    let co2 = count_bits_and_filter_till_theres_only_one_left(
        numbers,
        bits,
        |b, shift| b.bit(shift),
        |a, b| a <= b,
    );
    dbg!(oxygen);
    dbg!(co2);
    dbg!(oxygen * co2);
}

fn count_bits_and_filter_till_theres_only_one_left(
    mut numbers: Vec<u32>,
    total_bits: usize,
    criterion: impl Fn(u32, usize) -> bool,
    compare: impl Fn(usize, usize) -> bool,
) -> u32 {
    for shift in (0..total_bits).rev() {
        let occurence = numbers.iter().fold(0, |mut o, &num| {
            if criterion(num, shift) {
                o += 1;
            }
            o
        });
        let remaining = numbers.len() - occurence;
        dbg!(&occurence);
        dbg!(&remaining);
        numbers.retain(|&num| criterion(num, shift) ^ compare(occurence, remaining));
        dbg!(&numbers);
        if numbers.len() <= 1 {
            return numbers[0];
        }
    }
    panic!("there's more than one left! this is a bug!")
}
