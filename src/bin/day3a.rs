use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input3.txt").split_terminator('\n');
    
    let (entries, occurences) = input.fold(
        (0u32, vec![0u32; 0]),
        |(mut entries, mut occurences), x| {
            entries += 1;
            let num = u32::from_str_radix(x, 2).unwrap();
            occurences.resize(x.len(), 0);

            let mut mask = 1;
            for occurence in &mut occurences {
                if num & mask != 0 {
                    *occurence += 1;
                }
                mask <<= 1;
            }
            (entries, occurences)
        },
    );
    let (gamma, epsilon) =
        occurences
            .into_iter()
            .rev()
            .fold((0u32, 0u32), |(mut gamma, mut epsilon), occ| {
                gamma <<= 1;
                epsilon <<= 1;
                if occ < (entries / 2) {
                    gamma |= 1;
                } else {
                    epsilon |= 1;
                }
                (gamma, epsilon)
            });

    dbg!(gamma * epsilon);
}
