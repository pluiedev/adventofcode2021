#![feature(drain_filter)]
use std::cmp::Ordering;

use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input3.txt").split_terminator('\n');

    let bits = 12;
    let numbers: Vec<u32> = input.map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
    dbg!(&numbers);
    let threshold = (numbers.len() / 2) as u32;

    let oxygen = {
        let mut numbers = numbers.clone();
        for shift in (0..bits).rev() {
            dbg!(shift);
            dbg!(1<<shift);
            let occurence = numbers.iter().fold(0, |mut o, num| {
                if num & (1<<shift) != 0 {
                    o += 1;
                }
                o
            });
            match occurence.cmp(&(numbers.len() - occurence)) {
                Ordering::Greater | Ordering::Equal => numbers.retain(|num| num & (1<<shift) != 0),
                Ordering::Less => numbers.retain(|num| num & (1<<shift) == 0)
            }
            if numbers.len() <= 1 {
                break;
            }
        }
        dbg!(numbers[0])
    };
    let co2 = {
        let mut numbers = numbers.clone();
        for shift in (0..bits).rev() {
            dbg!(shift);
            dbg!(1<<shift);
            let occurence = numbers.iter().fold(0, |mut o, num| {
                if num & (1<<shift) == 0 {
                    o += 1;
                }
                o
            });
            match occurence.cmp(&(numbers.len() - occurence)) {
                Ordering::Greater => numbers.retain(|num| num & (1<<shift) != 0),
                Ordering::Less |  Ordering::Equal => numbers.retain(|num| num & (1<<shift) == 0)
            }
            if numbers.len() <= 1 {
                break;
            }
        }
        dbg!(numbers[0])
    };
    dbg!(oxygen * co2);
}
