use std::{fs::File, io::prelude::*, str::FromStr};

const INPUT: &'static str = "../inputs/day09.txt";

#[derive(Debug, Clone)]
struct Node {
    marble: usize,
    cw: usize,
    ccw: usize,
}

fn main() {
    let mut input = String::new();
    File::open(INPUT)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    // let input = "9 players; last marble is worth 25 points: high score is 32";
    // let input = "10 players; last marble is worth 1618 points: high score is 8317";
    // let input = "13 players; last marble is worth 7999 points: high score is 146373";
    // let input = "17 players; last marble is worth 1104 points: high score is 2764";
    let mut words = input.trim().split(" ");
    let players: usize = words.next().unwrap().parse().unwrap();
    let last_marble: usize = words.nth(5).unwrap().parse().unwrap();
    let last_marble = last_marble * 100;
    println!("players: {:?}", players);
    println!("last_marble: {:?}", last_marble);
    let mut scores = vec![0; players];
    let mut marbles = vec![
        Node {
            marble: 0,
            cw: 0,
            ccw: 0
        };
        1
    ];
    let mut player_index = 0;
    let mut current_index = 0;
    for marble in 1..=last_marble {
        // println!("marbles: {:?}", marbles);
        // println!("current_index: {:?}", current_index);
        if marble % 23 == 0 {
            let remove_index = ccw(
                ccw(
                    ccw(
                        ccw(
                            ccw(ccw(ccw(current_index, &marbles), &marbles), &marbles),
                            &marbles,
                        ),
                        &marbles,
                    ),
                    &marbles,
                ),
                &marbles,
            );
            let ccw = ccw(remove_index, &marbles);
            let cw = cw(remove_index, &marbles);
            marbles[ccw].cw = cw;
            marbles[cw].ccw = ccw;
            current_index = cw;
            scores[player_index] += marble + marbles[remove_index].marble;
        } else {
            let ccw = cw(current_index, &marbles);
            let cw = cw(ccw, &marbles);
            marbles.push(Node { marble, cw, ccw });
            let i = marbles.len() - 1;
            marbles[ccw].cw = i;
            marbles[cw].ccw = i;
            current_index = i;
        }
        player_index = (player_index + 1) % players;
    }
    println!("high score: {}", scores.iter().max().unwrap());
}

fn cw(from: usize, marbles: &[Node]) -> usize {
    marbles[from].cw
}

fn ccw(from: usize, marbles: &[Node]) -> usize {
    marbles[from].ccw
}
