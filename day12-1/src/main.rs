use std::{collections::HashMap, fs::File, io::prelude::*, str::FromStr};

const INPUT: &'static str = "../inputs/day12.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut lines = input.trim().lines();
    let mut initial_state: Vec<bool> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .chars()
        .map(|c| {
            if c == '#' {
                true
            } else if c == '.' {
                false
            } else {
                panic!()
            }
        })
        .collect();
    println!("{:?}", initial_state);

    lines.next().unwrap(); // empty line

    let mut rules = Vec::new();
    for line in lines {
        let mut words = line.split_whitespace();
        let pattern: Vec<bool> = words
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                if c == '#' {
                    true
                } else if c == '.' {
                    false
                } else {
                    panic!()
                }
            })
            .collect();
        let _ = words.next().unwrap();
        let result: bool = words
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                if c == '#' {
                    true
                } else if c == '.' {
                    false
                } else {
                    panic!()
                }
            })
            .next()
            .unwrap();
        rules.push((pattern, result));
    }
    let mut offsets = vec![0; 1];
    let mut states = vec![initial_state; 1];
    for _ in 0..20 {
        {
            let last_state = states.last_mut().unwrap();
            while last_state[..5].iter().any(|b| *b) {
                last_state.insert(0, false);
                *offsets.last_mut().unwrap() += 1;
            }
            while last_state[last_state.len() - 5..].iter().any(|b| *b) {
                last_state.push(false);
            }
        }
        let mut state = states.last().unwrap().clone();
        {
            let last_state = states.last().unwrap();
            'pot_loop: for pot_index in 2..(state.len() - 2) {
                for (pattern, result) in rules.iter() {
                    if &last_state[(pot_index - 2)..=(pot_index + 2)] == &pattern[..] {
                        state[pot_index] = *result;
                        continue 'pot_loop;
                    }
                }
                // state[pot_index] = false; // for the test input
                panic!(); // for full input
            }
        }
        states.push(state);
    }
    for state in states.iter() {
        for pot in state.iter() {
            if *pot {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    // println!("states {:?}", states);
    let offset = offsets.last().unwrap().clone();
    let sum = states
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .fold(
            0_i32,
            |acc, (i, b)| if *b { acc + i as i32 - offset } else { acc },
        );
    println!("sum {}", sum);
}
