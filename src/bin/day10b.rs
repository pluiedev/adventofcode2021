use std::{collections::VecDeque, fmt::Debug};

fn main() {
    let input = include_str!("../inputs/input10.txt").lines();

    let mut incomplete_scores = vec![];
    for line in input {
        let status = test(line);
        // dbg!(&status);
        if let LineStatus::Incomplete(mut stack) = status {
            let mut score = 0u64;
            // dbg!(&stack);
            while let Some(c) = stack.pop_back() {
                // dbg!(c);
                let pt = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                score = score * 5 + pt;
            }
            incomplete_scores.push(score);
        }
        // println!()
    }
    incomplete_scores.sort();
    dbg!(&incomplete_scores);
    let median = incomplete_scores.len() / 2;
    let median = incomplete_scores[median];
    dbg!(median);

    // code here
}

#[derive(Debug, Clone)]
enum LineStatus {
    Wellformed,
    Corrupted(char),
    Incomplete(VecDeque<char>),
}

fn test(line: &str) -> LineStatus {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        // println!("before: {:?}", stack);
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push_back(c);
            }
            ')' => match stack.pop_back() {
                Some('(') => {}
                Some(_) => return LineStatus::Corrupted(c),
                None => return LineStatus::Incomplete(stack),
            },
            ']' => match stack.pop_back() {
                Some('[') => {}
                Some(_) => return LineStatus::Corrupted(c),
                None => return LineStatus::Incomplete(stack),
            },
            '}' => match stack.pop_back() {
                Some('{') => {}
                Some(_) => return LineStatus::Corrupted(c),
                None => return LineStatus::Incomplete(stack),
            },
            '>' => match stack.pop_back() {
                Some('<') => {}
                Some(_) => return LineStatus::Corrupted(c),
                None => return LineStatus::Incomplete(stack),
            },
            o => return LineStatus::Corrupted(o),
        }
        // println!("after: {:?}", stack);
    }
    if stack.is_empty() {
        LineStatus::Wellformed
    } else {
        LineStatus::Incomplete(stack)
    }
}
