use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn to_priority(c: char) -> i32 {
    let v = c as i32;
    if v >= 97 {
        v - 96
    } else {
        v - 38
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let binding = reader.lines().filter_map(|x| x.ok()).chunks(3);
    let mismatches = binding
        .into_iter()
        .map(|mut three| {
            let mut a = three.next().unwrap().chars().collect::<HashSet<char>>();
            let b = three.next().unwrap().chars().collect::<HashSet<char>>();
            let c = three.next().unwrap().chars().collect::<HashSet<char>>();

            a.retain(|x| b.contains(x) & c.contains(x));
            a.into_iter().next()
        })
        .filter_map(|x| x.map(to_priority))
        .collect::<Vec<i32>>();
    println!("{:?}", mismatches.iter().sum::<i32>());
    Ok(())
}

fn main1() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mismatches = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|s| {
            s[0..s.len() / 2]
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&s[s.len() / 2..s.len()].chars().collect::<HashSet<char>>())
                .into_iter()
                .next()
                .copied()
        })
        .filter_map(|x| x.map(to_priority))
        .collect::<Vec<i32>>();
    println!("{:?}", mismatches.iter().sum::<i32>());
    Ok(())
}
