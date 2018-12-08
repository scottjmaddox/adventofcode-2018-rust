use std::{fs::File, io::prelude::*};

const INPUT: &'static str = "../inputs/day02.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    for line1 in input.lines() {
        for line2 in input.lines() {
            let dist =
                line1
                    .chars()
                    .zip(line2.chars())
                    .fold(0, |dist, (c1, c2)| if c1 == c2 { dist } else { dist + 1 });
            if dist == 1 {
                line1.chars().zip(line2.chars()).for_each(|(c1, c2)| {
                    if c1 == c2 {
                        print!("{}", c1);
                    }
                });
                println!("");
                return;
            }
        }
    }
}
