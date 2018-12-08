use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
};

const INPUT: &'static str = "../inputs/day01.txt";

fn main() {
    let mut file = File::open(INPUT).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut freqs = HashSet::new();

    let mut freq: isize = 0;
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            file = File::open(INPUT).unwrap();
            reader = BufReader::new(file);
            continue;
        }
        // println!("{}", line);
        let delta: isize = line.trim().parse().unwrap();
        freq += delta;
        if freqs.contains(&freq) {
            break;
        }
        freqs.insert(freq);
        line.clear();
    }
    println!("{}", freq);
}
