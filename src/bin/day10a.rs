use std::{fmt::Debug, collections::VecDeque};

fn main() {
    let input = include_str!("../inputs/input10.txt").lines();

    let mut sum = 0;
    for line in input {
        let status = test(line);
        dbg!(&status);
        if let LineStatus::Corrupted(found) = status {
            sum += match found {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                o => panic!("unexpected character: {}", o)
            }
        }
        println!()
    }
    dbg!(sum);

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