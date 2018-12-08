use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

const INPUT: &'static str = "../inputs/day01.txt";

fn main() {
    let file = File::open(INPUT).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut freq: isize = 0;
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }
        // println!("{}", line);
        let delta: isize = line.trim().parse().unwrap();
        freq += delta;
        line.clear();
    }
    println!("{}", freq);
}
