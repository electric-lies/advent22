use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: u8,
}

const ROPE_LENGTH: usize = 10;

#[derive(Debug)]
struct Rope {
    knots: [(i32, i32); ROPE_LENGTH],
}
fn parse(i: String) -> Instruction {
    let mut tup = i.split(" ");
    let direction = match tup.next().unwrap() {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!(),
    };
    let count: u8 = tup.next().unwrap().parse().unwrap();
    Instruction {
        direction: direction,
        count: count,
    }
}

fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let ops = BufReader::new(file)
        .lines()
        // .take(4)
        .filter_map(|x| x.ok())
        .map(parse);

    let mut rope = Rope {
        knots: [(0, 0); ROPE_LENGTH],
    };
    let mut places: HashSet<(i32, i32)> = HashSet::new();
    for instruction in ops {
        // println!("{:?}", instruction);
        let norm = match instruction.direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };
        for _ in 0..instruction.count {
            //head move
            rope.knots[0].0 += norm.0;
            rope.knots[0].1 += norm.1;

            for knot_number in 1..ROPE_LENGTH {
                let distance = (
                    rope.knots[knot_number - 1].0 - rope.knots[knot_number].0,
                    rope.knots[knot_number - 1].1 - rope.knots[knot_number].1,
                );
                if distance.0.abs() <= 1 && distance.1.abs() <= 1 {
                } else if distance.0.abs() == 2 {
                    if distance.1.abs() >= 1 {
                        rope.knots[knot_number].1 += distance.1 / distance.1.abs();
                    }
                    rope.knots[knot_number].0 += distance.0 / 2;
                } else if distance.1.abs() == 2 {
                    if distance.0.abs() >= 1 {
                        rope.knots[knot_number].0 += distance.0 / distance.0.abs();
                    }
                    rope.knots[knot_number].1 += distance.1 / 2;
                } else {
                    panic!()
                }
            }
            // println!("{:?}", rope);
            places.insert(rope.knots[ROPE_LENGTH - 1]);
        }
    }
    println!("plcaes: {:?}", places.len());
}
