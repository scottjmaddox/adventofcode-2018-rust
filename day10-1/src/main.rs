use std::{fs::File, io::prelude::*, str::FromStr};

const INPUT: &'static str = "../inputs/day10.txt";

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let pxstart = "position=<".len();
    let pxend = "position=<-41933".len();
    let pystart = "position=<-41933, ".len();
    let pyend = "position=<-41933,  10711".len();
    let vxstart = "position=<-41933,  10711> velocity=<".len();
    let vxend = "position=<-41933,  10711> velocity=< 4".len();
    let vystart = "position=<-41933,  10711> velocity=< 4, ".len();
    let vyend = "position=<-41933,  10711> velocity=< 4, -1".len();
    //     let input = "position=< 9,  1> velocity=< 0,  2>
    // position=< 7,  0> velocity=<-1,  0>
    // position=< 3, -2> velocity=<-1,  1>
    // position=< 6, 10> velocity=<-2, -1>
    // position=< 2, -4> velocity=< 2,  2>
    // position=<-6, 10> velocity=< 2, -2>
    // position=< 1,  8> velocity=< 1, -1>
    // position=< 1,  7> velocity=< 1,  0>
    // position=<-3, 11> velocity=< 1, -2>
    // position=< 7,  6> velocity=<-1, -1>
    // position=<-2,  3> velocity=< 1,  0>
    // position=<-4,  3> velocity=< 2,  0>
    // position=<10, -3> velocity=<-1,  1>
    // position=< 5, 11> velocity=< 1, -2>
    // position=< 4,  7> velocity=< 0, -1>
    // position=< 8, -2> velocity=< 0,  1>
    // position=<15,  0> velocity=<-2,  0>
    // position=< 1,  6> velocity=< 1,  0>
    // position=< 8,  9> velocity=< 0, -1>
    // position=< 3,  3> velocity=<-1,  1>
    // position=< 0,  5> velocity=< 0, -1>
    // position=<-2,  2> velocity=< 2,  0>
    // position=< 5, -2> velocity=< 1,  2>
    // position=< 1,  4> velocity=< 2,  1>
    // position=<-2,  7> velocity=< 2, -2>
    // position=< 3,  6> velocity=<-1, -1>
    // position=< 5,  0> velocity=< 1,  0>
    // position=<-6,  0> velocity=< 2,  0>
    // position=< 5,  9> velocity=< 1, -2>
    // position=<14,  7> velocity=<-2,  0>
    // position=<-3,  6> velocity=< 2, -1>
    // ";
    // let pxstart = 10;
    // let pxend = 12;
    // let pystart = 14;
    // let pyend = 16;
    // let vxstart = 28;
    // let vxend = 30;
    // let vystart = 32;
    // let vyend = 34;
    let mut pxs = Vec::new();
    let mut pys = Vec::new();
    let mut vxs = Vec::new();
    let mut vys = Vec::new();
    for line in input.trim().lines() {
        // println!("{}", line);
        let px: i32 = line[pxstart..pxend].trim().parse().unwrap();
        let py: i32 = line[pystart..pyend].trim().parse().unwrap();
        let vx: i32 = line[vxstart..vxend].trim().parse().unwrap();
        let vy: i32 = line[vystart..vyend].trim().parse().unwrap();
        // println!("{}, {}, {}, {}", px, py, vx, vy);
        pxs.push(px);
        pys.push(py);
        vxs.push(vx);
        vys.push(vy);
    }
    let mut step = 0;
    let mut min_step = 0;
    let mut min_area = u64::max_value();
    loop {
        let area = area(pxs.as_slice(), pys.as_slice());
        if area < min_area {
            min_step = step;
            min_area = area;
        }
        println!(
            "step: {}, min_step: {}, min_area: {}",
            step, min_step, min_area
        );
        if step == 10519 {
            plot(pxs.as_slice(), pys.as_slice());
            return;
        }
        step += 1;
        for i in 0..pxs.len() {
            pxs[i] += vxs[i];
            pys[i] += vys[i];
        }
    }
}

fn area(pxs: &[i32], pys: &[i32]) -> u64 {
    (pxs.iter().max().unwrap() - pxs.iter().min().unwrap()) as u64
        * (pys.iter().max().unwrap() - pys.iter().min().unwrap()) as u64
}

fn plot(pxs: &[i32], pys: &[i32]) {
    for y in *pys.iter().min().unwrap()..=*pys.iter().max().unwrap() {
        'x: for x in *pxs.iter().min().unwrap()..=*pxs.iter().max().unwrap() {
            for i in 0..pxs.len() {
                if x == pxs[i] && y == pys[i] {
                    print!("#");
                    continue 'x;
                }
            }
            print!(".");
        }
        println!("");
    }
    println!("");
}
