use std::{fs::File, io::prelude::*};

const INPUT: &'static str = "../inputs/day05.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let input = input.trim();

    // for A through Z
    let mut shortest_length = usize::max_value();
    for v in 65_u8..=90 {
        let c: char = v.into();
        let length = reacted_polymer_length(input.chars().filter(|v| !v.eq_ignore_ascii_case(&c)));
        if length < shortest_length {
            shortest_length = length;
        }
    }

    println!("answer: {}", shortest_length);
}

fn reacted_polymer_length<I: Iterator<Item = char>>(input: I) -> usize {
    let mut stack = Vec::new();
    for c in input {
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
