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
        let x: u32 = x_str.parse().unwrap();
        // println!("y_str {:?}", y_str);
        let y: u32 = y_str.parse().unwrap();
        xs.push(x);
        ys.push(y);
    }
    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();
    let x_count = max_x - min_x + 1;
    let stride = x_count as usize;
    let y_count = max_y - min_y + 1;
    let mut grid = vec![(usize::max_value(), u32::max_value()); (y_count * x_count) as usize];
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            for i in 0..xs.len() {
                let grid_i = (y - min_y) as usize * stride + (x - min_x) as usize;
                let x_dist = if xs[i] > x { xs[i] - x } else { x - xs[i] };
                let y_dist = if ys[i] > y { ys[i] - y } else { y - ys[i] };
                let dist = x_dist + y_dist;
                if dist < grid[grid_i].1 {
                    // New closest location.
                    grid[grid_i] = (i, dist);
                } else if dist == grid[grid_i].1 {
                    // Location is equally close.
                    // `usize::max_value()` is a sentinal value.
                    grid[grid_i] = (usize::max_value(), dist);
                }
            }
        }
    }
    let mut area_sizes = vec![0_usize; xs.len()];
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let grid_i = (y - min_y) as usize * stride + (x - min_x) as usize;
            let i = grid[grid_i].0;
            if i != usize::max_value() {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    area_sizes[i] = usize::max_value();
                } else {
                    area_sizes[i] = area_sizes[i].saturating_add(1);
                }
            }
        }
    }
    println!("area_sizes {:?}", area_sizes);
    println!(
        "answer {:?}",
        area_sizes
            .iter()
            .filter(|v| **v != usize::max_value())
            .max()
            .unwrap()
    );
}
