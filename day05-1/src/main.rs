use std::{fs::File, io::prelude::*};

const INPUT: &'static str = "../inputs/day05.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    println!("answer: {}", reacted_polymer_length(input.as_str()));
}

fn reacted_polymer_length(input: &str) -> usize {
    let mut stack = Vec::new();
    for c in input.trim().chars() {
        if stack.last().is_some() {
            let last = *stack.last().unwrap();
            if c != last && c.eq_ignore_ascii_case(&last) {
                // println!("{} and {} react", last, c);
                stack.pop();
                continue;
            } else {
                // println!("{} and {} don't react", last, c);
            }
        }
        stack.push(c);
    }
    stack.len()
}
