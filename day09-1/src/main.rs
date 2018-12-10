use std::{fs::File, io::prelude::*, str::FromStr};

const INPUT: &'static str = "../inputs/day09.txt";

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
    println!("players: {:?}", players);
    println!("last_marble: {:?}", last_marble);
    let mut scores = vec![0; players];
    let mut marbles = vec![0; 1];
    let mut player_index = 0;
    let mut current_marble_index = 0;
    for marble in 1..=last_marble {
        // println!("marbles: {:?}", marbles);
        // println!("current_marble_index: {:?}", current_marble_index);
        if marble % 23 == 0 {
            let remove_index = if current_marble_index >= 7 {
                current_marble_index - 7
            } else {
                current_marble_index + marbles.len() - 7
            };
            let removed_marble = marbles.remove(remove_index);
            scores[player_index] += marble + removed_marble;
            current_marble_index = remove_index;
        } else {
            let insert_index = (current_marble_index + 2) % marbles.len();
            marbles.insert(insert_index, marble);
            current_marble_index = insert_index;
        }
        player_index = (player_index + 1) % players;
    }
    println!("high score: {}", scores.iter().max().unwrap());
}
