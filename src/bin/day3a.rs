use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input3.txt")
        .split_terminator('\n')
        .collect_vec();

    let entries = input.len();

    let occurences = input.iter().fold(vec![0usize; 0], |mut occurences, x| {
        let num = u32::from_str_radix(x, 2).unwrap();
        occurences.resize(x.len(), 0);

        for (shift, occurence) in occurences.iter_mut().enumerate() {
            if num.bit(shift) {
                *occurence += 1;
            }
        }
        occurences
    });
    let (gamma, epsilon) =
        occurences
            .into_iter()
            .rev()
            .fold((0u32, 0u32), |(mut gamma, mut epsilon), occ| {
                gamma <<= 1;
                epsilon <<= 1;
                let remaining = entries - occ;
                if occ < remaining {
                    gamma |= 1;
                } else {
                    epsilon |= 1;
                }
                (gamma, epsilon)
            });

    dbg!(gamma * epsilon);
}
