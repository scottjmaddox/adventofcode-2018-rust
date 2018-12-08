extern crate regex;

use std::{collections::HashMap, fs::File, io::prelude::*};

const INPUT: &'static str = "../inputs/day03.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let re = regex::Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    let mut sparse_counts = HashMap::new();

    for line in input.lines() {
        // println!("line: \"{}\"", line);
        let cap = re.captures(line).unwrap();
        // println!("cap: {:?}", cap);
        let _claim_id: usize = cap[1].parse().unwrap();
        let x: usize = cap[2].parse().unwrap();
        let y: usize = cap[3].parse().unwrap();
        let w: usize = cap[4].parse().unwrap();
        let h: usize = cap[5].parse().unwrap();
        for i in x..(x + w) {
            for j in y..(y + h) {
                if sparse_counts.contains_key(&(i, j)) {
                    *sparse_counts.get_mut(&(i, j)).unwrap() += 1;
                } else {
                    sparse_counts.insert((i, j), 1);
                }
            }
        }
    }
    'line_loop: for line in input.lines() {
        // println!("line: \"{}\"", line);
        let cap = re.captures(line).unwrap();
        // println!("cap: {:?}", cap);
        let claim_id: usize = cap[1].parse().unwrap();
        let x: usize = cap[2].parse().unwrap();
        let y: usize = cap[3].parse().unwrap();
        let w: usize = cap[4].parse().unwrap();
        let h: usize = cap[5].parse().unwrap();
        for i in x..(x + w) {
            for j in y..(y + h) {
                if *sparse_counts.get(&(i, j)).unwrap() != 1 {
                    continue 'line_loop;
                }
            }
        }
        println!("uncontended claim_id: {}", claim_id);
        return;
    }
}
