use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

const INPUT: &'static str = "../inputs/day02.txt";

fn main() {
    let file = File::open(INPUT).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut doubles: usize = 0;
    let mut triples: usize = 0;
    let mut letter_freqs = HashMap::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        // println!("line: \"{}\"", line);
        for c in line.chars() {
            if letter_freqs.contains_key(&c) {
                *letter_freqs.get_mut(&c).unwrap() += 1;
            } else {
                letter_freqs.insert(c, 1_usize);
            }
        }
        // println!("letter_freqs: {:?}", letter_freqs);
        if letter_freqs.values().any(|v| *v == 2) {
            doubles += 1;
        }
        if letter_freqs.values().any(|v| *v == 3) {
            triples += 1;
        }

        letter_freqs.clear();
        line.clear();
    }
    println!("checksum: {}", doubles * triples);
}
