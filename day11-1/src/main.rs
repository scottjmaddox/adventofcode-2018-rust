use std::{fs::File, io::prelude::*, str::FromStr};

const INPUT: &'static str = "../inputs/day10.txt";

fn main() {
    let grid_serial_number = 1308;
    // let grid_serial_number = 18;
    // let answer_x = 33;
    // let answer_y = 45;
    // let answer_power = 29;

    let mut grid = vec![0; 300 * 300];

    for y in 1..=300 {
        for x in 1..=300 {
            // let grid_serial_number = 8;
            // let x = 3_i32;
            // let y = 5_i32;
            let rack_id = x + 10;
            let mut power_level = rack_id * y;
            power_level += grid_serial_number;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            power_level -= 5;
            grid[(y as usize - 1) * 300 + (x as usize - 1)] = power_level;
        }
    }

    let mut total_grid = vec![0; 300 * 300];
    let mut max_total = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for y in 1..=298 {
        for x in 1..=298 {
            let total = grid[(y as usize - 1) * 300 + (x as usize - 1)]
                + grid[(y as usize - 1) * 300 + (x as usize - 0)]
                + grid[(y as usize - 1) * 300 + (x as usize + 1)]
                + grid[(y as usize - 0) * 300 + (x as usize - 1)]
                + grid[(y as usize - 0) * 300 + (x as usize - 0)]
                + grid[(y as usize - 0) * 300 + (x as usize + 1)]
                + grid[(y as usize + 1) * 300 + (x as usize - 1)]
                + grid[(y as usize + 1) * 300 + (x as usize - 0)]
                + grid[(y as usize + 1) * 300 + (x as usize + 1)];
            if total > max_total {
                max_total = total;
                max_x = x;
                max_y = y;
            }
        }
    }
    println!("{},{} with {}", max_x, max_y, max_total);
}
