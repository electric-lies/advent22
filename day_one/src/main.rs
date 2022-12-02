use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut sums = reader
        .lines()
        .filter_map(|x| x.ok())
        .group_by(|content| content.is_empty())
        .into_iter()
        .map(|(_, l)| l.into_iter().map(|s| s.parse::<i32>().unwrap_or(0)).sum())
        .collect::<Vec<i32>>();

    sums.sort();
    sums.reverse();
    println!("{:?}, {}", sums, sums[0..3].iter().sum::<i32>());
    Ok(())
}
