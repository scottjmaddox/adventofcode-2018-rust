use std::{fs::File, io::prelude::*};

const INPUT: &'static str = "../inputs/day06.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for line in input.lines() {
        // println!("line {:?}", line);
        let mut it = line.split(',');
        let x_str = it.next().unwrap().trim();
        let y_str = it.next().unwrap().trim();
        // println!("x_str {:?}", x_str);
        let x: i32 = x_str.parse().unwrap();
        // println!("y_str {:?}", y_str);
        let y: i32 = y_str.parse().unwrap();
        xs.push(x);
        ys.push(y);
    }
    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();
    let mut area = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut total_dist = 0;
            for i in 0..xs.len() {
                total_dist += dist(xs[i], x, ys[i], y);
            }
            if total_dist < 10000 {
                area += 1;
            }
        }
    }
    println!("answer {:?}", area);
}

fn dist(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let x_dist = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let y_dist = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    x_dist + y_dist
}
