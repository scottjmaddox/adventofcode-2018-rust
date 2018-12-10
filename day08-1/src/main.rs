use std::{fs::File, io::prelude::*, str::FromStr};

const INPUT: &'static str = "../inputs/day08.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut values = input.trim().split(" ").map(|s| u32::from_str(s).unwrap());
    let mut acc = 0;
    visit_metadata(&mut values, &mut |v| {
        acc += v;
    });
    println!("acc: {}", acc);
}

fn visit_metadata<I, F>(values: &mut I, f: &mut F)
where
    I: Iterator<Item = u32>,
    F: FnMut(u32),
{
    let child_node_count = values.next().unwrap();
    let metadata_entry_count = values.next().unwrap();
    for _ in 0..child_node_count {
        visit_metadata(values, f);
    }
    for _ in 0..metadata_entry_count {
        f(values.next().unwrap());
    }
}
