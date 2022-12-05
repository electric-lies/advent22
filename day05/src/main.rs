use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::streaming::is_not;
use nom::character::complete::newline;
use nom::character::streaming::char;
use nom::multi::{many1, many_till};
use nom::sequence::delimited;
use nom::IResult;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_moves(input: &str) -> IResult<&str, Vec<(Vec<&str>, char)>> {
    let empty = tag("    ");
    let full = delimited(alt((tag("["), tag(" ["))), is_not("]"), char(']'));
    let line = many_till(alt((full, empty)), newline);
    many1(line)(input)
}

fn main() {
    let file = File::open("setup.txt").expect("did not found file");
    let mut buf: String = "".into();
    BufReader::new(file)
        .read_to_string(&mut buf)
        .expect("failed to allocate buffer from file");
    let mut lines = parse_moves(&buf).unwrap().1;

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..9 {
        stacks.push(Vec::<char>::new());
    }

    lines.reverse();
    // println!("{:?}", stacks);
    for line in lines {
        let mut i = line.0.iter();
        for ind in 0..9 {
            let v = *i.next().expect("less then 9 crates in line");
            if v != "    " {
                let val = v.chars().next().unwrap();
                stacks.get_mut(ind).expect("hmmm").push(val);
            }
        }
    }

    let file = File::open("move.txt").expect("did not found file");
    let moves: Vec<(i8, i8, i8)> = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|s| s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|v| {
            (
                v.get(1).unwrap().parse::<i8>().unwrap(),
                v.get(3).unwrap().parse::<i8>().unwrap(),
                v.get(5).unwrap().parse::<i8>().unwrap(),
            )
        })
        .collect();

    let mut new_stacks = make_moves_two(stacks, moves);

    println!(
        "{:?}",
        new_stacks
            .iter_mut()
            .map(|v| v.pop().unwrap())
            .collect::<String>()
    )
}

fn make_moves_one(mut stacks: Vec<Vec<char>>, moves: Vec<(i8, i8, i8)>) -> Vec<Vec<char>> {
    for mov in moves {
        for _ in 0..mov.0 {
            let val = stacks.get_mut((mov.1 - 1) as usize).unwrap().pop();
            let pack = match val {
                Some(n) => n,
                None => break,
            };

            stacks.get_mut((mov.2 - 1) as usize).unwrap().push(pack)
        }
    }

    stacks
}

fn make_moves_two(mut stacks: Vec<Vec<char>>, moves: Vec<(i8, i8, i8)>) -> Vec<Vec<char>> {
    for mov in moves {
        let mut vals: Vec<Option<char>> = Vec::new();
        for _ in 0..mov.0 {
            vals.push(stacks.get_mut((mov.1 - 1) as usize).unwrap().pop());
        }

        vals.reverse();

        for val in vals.iter() {
            let pack = match val {
                Some(n) => n,
                None => break,
            };

            stacks.get_mut((mov.2 - 1) as usize).unwrap().push(*pack)
        }
    }
    stacks
}
