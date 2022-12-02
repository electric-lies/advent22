use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut curr_sum = 0;
    let mut sums = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                if content.is_empty() {
                    sums.push(curr_sum);
                    curr_sum = 0
                } else {
                    curr_sum += content.parse::<i32>().unwrap();
                }
            }
            Err(_) => todo!(),
        }
    }
    sums.sort();
    sums.reverse();
    println!("{:?}, {}", sums, sums[0..3].iter().sum::<i32>());
    Ok(())
}
