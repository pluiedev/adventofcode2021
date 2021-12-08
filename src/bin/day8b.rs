use std::collections::HashMap;

use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input8.txt").lines();

    let count = input.fold(0, |mut acc, s| {
        let (digits, output) = s.split_once(" | ").unwrap();
        // analysis
        let digits = digits
            .split_ascii_whitespace()
            .map(|s| {
                dbg!(s);
                let ss = s
                    .bytes()
                    .fold(SevenSeg::empty(), |acc, b| acc | SevenSeg::from_ch(b));
                (s.len(), ss)
            })
            .sorted_by_key(|(len, _)| *len)
            .map(|(_, ss)| ss)
            .collect_vec();
        dbg!(&digits);

        // special numbers and ranges
        let one = digits[0];
        let seven = digits[1];
        let four = digits[2];
        let eight = digits[9];
        let five_segmented = &digits[3..=5];
        let six_segmented = &digits[6..=8];

        let a = seven - one;
        let bd = four - one;

        // find the 6-segmented number that entirely contains four (bcdf)
        // expects nine (abcdfg)
        let nine = *six_segmented
            .iter()
            .find(|x| x.contains(four))
            .unwrap();
        let g = nine - four - a;
        
        // zero is the only 6-segmented number that only has b but not d set
        // given we know bd, we can find d, then deduce b
        let zero = *six_segmented
            .iter()
            .find(|x| !x.contains(bd))
            .unwrap();
        let d = bd - zero;
        let b = bd - d;

        // five is the only 5-segmented number that has both b and d set
        // we can find f with it
        let five = *five_segmented
            .iter()
            .find(|x| x.contains(bd))
            .unwrap(); 
        let f = five - a - bd - g;
        let c = seven - f - a;
        let e = eight - nine;

        dbg!((a, b, c, d, e, f, g));

        let three = five - b | c;
        let two = three - f | e;
        let six = five | e;
        let map = HashMap::from([
            (zero, 0),
            (one, 1),
            (two, 2),
            (three, 3),
            (four, 4),
            (five, 5),
            (six, 6),
            (seven, 7),
            (eight, 8),
            (nine, 9),
        ]);
        // decode
        let mut total = 0;
        for digit in output.split_ascii_whitespace() {
            dbg!(acc);
            total *= 10;
            let ss = digit
                .bytes()
                .fold(SevenSeg::empty(), |acc, b| acc | SevenSeg::from_ch(b));
            total += map[&ss];
        }
        acc + total
    });
    dbg!(count);
    // code here
}

bitflags::bitflags! {
    struct SevenSeg: u8 {
        const A = 1;
        const B = 2;
        const C = 4;
        const D = 8;
        const E = 16;
        const F = 32;
        const G = 64;
    }
}
impl SevenSeg {
    fn from_ch(ch: u8) -> Self {
        match ch {
            b'a' => Self::A,
            b'b' => Self::B,
            b'c' => Self::C,
            b'd' => Self::D,
            b'e' => Self::E,
            b'f' => Self::F,
            b'g' => Self::G,
            _ => panic!(),
        }
    }
}
