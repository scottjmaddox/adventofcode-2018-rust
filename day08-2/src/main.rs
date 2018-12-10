use std::{fs::File, io::prelude::*, str::FromStr};

const INPUT: &'static str = "../inputs/day08.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut values = input.trim().split(" ").map(|s| usize::from_str(s).unwrap());
    let answer = node_value(&mut values);
    println!("answer: {}", answer);
}

fn node_value<I>(values: &mut I) -> usize
where
    I: Iterator<Item = usize>,
{
    let child_node_count = values.next().unwrap();
    let metadata_entry_count = values.next().unwrap();
    if child_node_count == 0 {
        values.take(metadata_entry_count).sum()
    } else {
        let node_values: Vec<usize> = (0..child_node_count).map(|_| node_value(values)).collect();
        values
            .take(metadata_entry_count)
            .map(|i| {
                if i - 1 < node_values.len() {
                    node_values[i - 1]
                } else {
                    0
                }
            }).sum()
    }
}
