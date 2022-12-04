use std::fs::File;
use std::io::{prelude::*, BufReader};

fn range(tup: (i8, i8)) -> u128 {
    (tup.0 as u8..tup.1 as u8).fold(0, |acc, b| acc | 1u128 << b)
}

fn one(tup: ((i8, i8), (i8, i8))) -> u16 {
    let (l, r) = tup;
    (((l.0 >= r.0) & (l.0 <= r.1) & (l.1 >= r.0) & (l.1 <= r.1))
        | ((r.0 >= l.0) & (r.0 <= l.1) & (r.1 >= l.0) & (r.1 <= l.1))) as u16
}

fn two(tup: ((i8, i8), (i8, i8))) -> u16 {
    let (l, r) = tup;
    (((l.0 >= r.0) & (l.0 <= r.1) | (l.1 >= r.0) & (l.1 <= r.1))
        | ((r.0 >= l.0) & (r.0 <= l.1) | (r.1 >= l.0) & (r.1 <= l.1))) as u16
}

fn parse(s: String) -> ((i8, i8), (i8, i8)) {
    let mut parts = s.split(&['-', ',']);
    (
        (
            parts.next().unwrap().parse::<i8>().unwrap(),
            parts.next().unwrap().parse::<i8>().unwrap(),
        ),
        (
            parts.next().unwrap().parse::<i8>().unwrap(),
            parts.next().unwrap().parse::<i8>().unwrap(),
        ),
    )
}

fn main() {
    println!("Hello, world!");
    let file = File::open("src/input.src").unwrap();
    let reader = BufReader::new(file);
    let covers = reader.lines().filter_map(|x| x.ok()).map(parse);
    println!(
        "one: {}, two :{}",
        covers.map(one).sum::<u16>(),
        covers.map(two).sum::<u16>()
    );
}
