extern crate regex;

use std::{collections::HashMap, fs::File, io::prelude::*};

const INPUT: &'static str = "../inputs/day07.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let re =
        regex::Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    let mut deps = HashMap::new();
    let mut inv_deps = HashMap::new();
    for line in input.lines() {
        // println!("{}", line);
        let cap = re.captures(line).unwrap();
        let c1: char = cap[1].parse().unwrap();
        let c2: char = cap[2].parse().unwrap();
        if !deps.contains_key(&c1) {
            deps.insert(c1, Vec::new());
        }
        if !deps.contains_key(&c2) {
            deps.insert(c2, Vec::new());
        }
        // c2 depends on c1
        deps.get_mut(&c2).unwrap().push(c1);
        if !inv_deps.contains_key(&c1) {
            inv_deps.insert(c1, Vec::new());
        }
        if !inv_deps.contains_key(&c2) {
            inv_deps.insert(c2, Vec::new());
        }
        // c1 depends inversly on c2
        inv_deps.get_mut(&c1).unwrap().push(c2);
    }
    let mut ready = Vec::new();
    ready.extend(
        deps.iter()
            .filter_map(|(k, v)| if v.is_empty() { Some(*k) } else { None }),
    );
    let mut completed = Vec::new();
    while !ready.is_empty() {
        ready.sort_unstable_by(|a, b| b.cmp(a));
        let c = ready.pop().unwrap();
        completed.push(c);
        'inv_dep_loop: for inv_dep in inv_deps.get(&c).unwrap() {
            if !completed.contains(inv_dep) && !ready.contains(inv_dep) {
                for dep in deps.get(inv_dep).unwrap() {
                    if !completed.contains(dep) {
                        continue 'inv_dep_loop;
                    }
                }
                ready.push(*inv_dep);
            }
        }
    }
    println!("Unsorted:");
    for c in inv_deps.keys() {
        print!("{}", c);
    }
    println!("\nAnswer:");
    for c in completed.iter() {
        print!("{}", c);
    }
}
