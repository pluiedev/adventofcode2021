use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input9.txt");

    let (columns, numbers) = input
        .split_whitespace()
        .fold((0usize, vec![]), |(_, mut vec), x| {
            vec.extend(x.bytes().map(|x| x - 0x30));
            (x.len(), vec)
        });
    dbg!(&numbers);
    let rows = numbers.len() / columns;
    dbg!((rows, columns));

    let mut cnt = 0;
    let mut risk_level = 0;
    for (index, &x) in numbers.iter().enumerate() {
        let row = index / columns;
        let col = index % columns;
        let mut rbc = false;
        // dbg!((index,x));
        if let Some(left) = index.checked_sub(1) {
            if (left / columns) == row {
                let left = numbers[left];
                // dbg!(left);
                if left <= x {
                    continue;
                }
            }
        }
        if let Some(right) = index.checked_add(1) {
            if (right % columns) > 0 {
                let right = numbers[right];
                // dbg!(right);
                if right <= x {
                    continue;
                }
            }
        }
        if let Some(top) = index.checked_sub(columns) {
            let top = numbers[top];
            // dbg!(top);
            if top <= x {
                continue;
            }
        }
        if let Some(bottom) = index.checked_add(columns)  {
            if bottom / columns < rows {
                let bottom = numbers[bottom];
                // dbg!(bottom);
                if bottom <= x {
                    continue;
                }
            } else {
                println!("bottom: [{}] ({}, {}) {}", index, row+1, col+1, x);
            }
        }
        if rbc {
            println!("[{}] ({}, {}) {}", index, row+1, col+1, x);
        }
        cnt += 1;
        risk_level += (x + 1) as u64;
    }
    dbg!((cnt, risk_level));
}
    
