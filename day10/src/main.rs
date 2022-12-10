use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::is_alphanumeric;
use nom::sequence::preceded;
use nom::{bytes::complete::tag, combinator::map};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const LINE_LENGTH: i32 = 40;

fn parse(i: String) -> (i32, i32) {
    alt((
        map(tag("noop"), |_| (1, 0)),
        map(
            preceded(
                tag::<&str, &str, ()>("addx "),
                take_while1(|x: char| is_alphanumeric(x as u8) || x == '-'),
            ),
            |s: &str| (2, s.parse().unwrap()),
        ),
    ))(&i)
    .unwrap()
    .1
}

#[test]
fn test_parse_noop() -> () {
    assert_eq!(parse("noop".to_string()), (1, 0))
}
#[test]
fn test_parse_addx() -> () {
    assert_eq!(parse("addx -2".to_string()), (2, -2))
}

fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let ops = BufReader::new(file)
        .lines()
        // .take(4)
        .filter_map(|x| x.ok())
        .map(parse)
        .collect::<Vec<(i32, i32)>>();
    println!("{:?}", part1(ops.clone()));

    let buffer = part2(ops);

    for line in buffer {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn part2(ops: Vec<(i32, i32)>) -> Vec<Vec<char>> {
    let mut round = 0i32;
    let mut state = 1;
    let mut buffer = Vec::<Vec<char>>::new();
    let mut curr_line = Vec::<char>::new();
    for op in ops {
        for _ in 0..op.0 {
            // println!("{}-{}", round, state);
            if (round - state).abs() <= 1 {
                curr_line.push('#');
            } else {
                curr_line.push('.');
            }
            round += 1;
            if round % LINE_LENGTH == 0 {
                buffer.push(curr_line);
                curr_line = Vec::<char>::new();
                round = 0;
            }
        }
        state += op.1;
    }
    buffer
}

fn part1(ops: Vec<(i32, i32)>) -> Vec<i32> {
    let mut round = 0;
    let mut state = 1;
    let mut interst = Vec::<i32>::new();
    let mut flag = false;
    for op in ops {
        for _ in 0..op.0 {
            if round % 20 == 0 {
                if flag {
                    println!("{:?} - {} - {}", op, state, round);
                    interst.push(state * round);
                }
                flag = !flag;
            }
            round += 1;
        }
        state += op.1;
    }
    interst
}
