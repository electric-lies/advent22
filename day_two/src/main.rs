use std::fs::File;
use std::io::{self, prelude::*, BufReader};

enum enemy {
    a,
    b,
    c,
}

enum player {
    x,
    y,
    z,
}
fn parse(input: String) -> (enemy, player) {
    let e = match input.chars().nth(0).unwrap() {
        'A' => enemy::a,
        'B' => enemy::b,
        'C' => enemy::c,
        _ => panic!(),
    };

    let p = match input.chars().nth(2).unwrap() {
        'X' => player::x,
        'Y' => player::y,
        'Z' => player::z,
        _ => panic!(),
    };

    (e, p)
}

fn score(moves: (enemy, player)) -> i32 {
    match moves {
        (enemy::a, player::x) => 4,
        (enemy::a, player::y) => 8,
        (enemy::a, player::z) => 3,
        (enemy::b, player::x) => 1,
        (enemy::b, player::y) => 5,
        (enemy::b, player::z) => 9,
        (enemy::c, player::x) => 7,
        (enemy::c, player::y) => 2,
        (enemy::c, player::z) => 6,
    }
}

fn score2(moves: (enemy, player)) -> i32 {
    match moves {
        (enemy::a, player::x) => 3,
        (enemy::a, player::y) => 4,
        (enemy::a, player::z) => 8,
        (enemy::b, player::x) => 1,
        (enemy::b, player::y) => 5,
        (enemy::b, player::z) => 9,
        (enemy::c, player::x) => 2,
        (enemy::c, player::y) => 6,
        (enemy::c, player::z) => 7,
    }
}

fn main() {
    println!("Hello, world!");
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let turns = reader.lines().filter_map(|x| x.ok()).map(parse);
    let scores = turns.map(score2);
    print!("{}", scores.sum::<i32>())
}
