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
    let mut last_state = initial_state;
    let mut last_offset = 0_i64;
    normalize(&mut last_state, &mut last_offset);
    let mut offset = last_offset;
    let mut state;
    let mut gen = 0;
    for _ in 0..50000000000_u64 {
        gen += 1;
        offset = last_offset;
        state = last_state.clone();
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
        normalize(&mut state, &mut offset);
        if last_state == state {
            println!("breaking early after generation {}", gen);
            break;
        }
        for pot in state.iter() {
            print!("{}", if *pot { '#' } else { '.' });
        }
        println!("\n");
        last_offset = offset;
        last_state = state;
    }
    // println!("states {:?}", states);
    let adjusted_offset = offset + (offset - last_offset) * (50000000000_i64 - gen);
    let sum = last_state.iter().enumerate().fold(0, |acc, (i, b)| {
        if *b {
            acc + i as i64 - adjusted_offset
        } else {
            acc
        }
    });
    println!("sum {}", sum);
}

fn normalize(state: &mut Vec<bool>, offset: &mut i64) {
    while state[..5].iter().any(|b| *b) {
        state.insert(0, false);
        *offset += 1;
    }
    while state[..5].iter().all(|b| !*b) {
        state.remove(0);
        *offset -= 1;
    }
    while state[state.len() - 6..].iter().any(|b| *b) {
        state.push(false);
    }
}
