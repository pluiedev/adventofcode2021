use std::{collections::VecDeque, fmt::Debug};

use adventofcode2021::prelude::*;

fn main() {
    let input = include_str!("../inputs/input11.txt").lines();

    let mut input = input
        .map(|x| x.bytes().map(|b| b - 0x30).collect_vec())
        .collect_vec();
    let size = input.len();

    let mut queue = VecDeque::new();
    let mut memory = vec![vec![false; size]; size];
    let mut i = 0;
    dbg!(size);
    loop {
        i += 1;
        // println!("iteration {} - started", i);
        // for line in &input {
        //     println!("{:?}", line);
        // }
        // for mem in &memory {
        //     println!("{:?}", mem);
        // }
        // println!();
        for (y, line) in input.iter_mut().enumerate() {
            for (x, num) in line.iter_mut().enumerate() {
                *num += 1;
                if *num > 9 {
                    memory[y][x] = true;
                    propagate(&mut queue, x, y, size);
                    *num = 0;
                }
            }
        }
        // println!("iteration {} - queued", i);
        // for line in &input {
        //     println!("{:?}", line);
        // }
        // for mem in &memory {
        //     println!("{:?}", mem);
        // }
        // println!("{:?}", queue);
        // println!();
        while let Some(PropagationEvent(x, y)) = queue.pop_front() {
            // dbg!((x, y));
            if !memory[y][x] {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    memory[y][x] = true;
                    propagate(&mut queue, x, y, size);
                    input[y][x] = 0;
                }
            }
        } 
        if i % 10 == 0 {
            println!("iteration {} - ended", i);
            for line in &input {
                println!("{:?}", line);
            }
            // for mem in &memory {
            //     println!("{:?}", mem);
            // }
            println!();
        }     
        
        let mut all_flashes = true;
        // clear
        for e in &mut memory {
            for c in e {
                if all_flashes && !*c {
                    all_flashes = false;
                }
                *c = false;
            }
        }
        if all_flashes {
            dbg!(i);
            break;
        }
    }
}

struct PropagationEvent(usize, usize);
impl Debug for PropagationEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.0).field(&self.1).finish()
    }
}

fn propagate(queue: &mut VecDeque<PropagationEvent>, x: usize, y: usize, size: usize) {
    if y > 0 {
        if x > 0 {
            queue.push_back(PropagationEvent(x - 1, y - 1));
        }
        queue.push_back(PropagationEvent(x, y - 1));
        if x + 1 < size {
            queue.push_back(PropagationEvent(x + 1, y - 1));
        }
    }
    if x > 0 {
        queue.push_back(PropagationEvent(x - 1, y));
    }
    if x + 1 < size {
        queue.push_back(PropagationEvent(x + 1, y));
    }
    if y + 1 < size {
        if x > 0 {
            queue.push_back(PropagationEvent(x - 1, y + 1));
        }
        queue.push_back(PropagationEvent(x, y + 1));
        if x + 1 < size {
            queue.push_back(PropagationEvent(x + 1, y + 1));
        }
    }
}