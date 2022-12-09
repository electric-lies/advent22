use camino::Utf8PathBuf;
use nom::bytes::complete::take_while1;
use nom::character::is_alphabetic;
use nom::sequence::preceded;
use nom::{bytes::complete::tag, combinator::map, IResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Cd(String),
    Ls,
    File(i32),
    Dir(String),
}

// enum Node {dir(Vec<Node>, file(i32))}

#[test]
fn check_parse() -> () {
    assert_eq!(
        special_parse("$ cd ..").unwrap().1,
        Line::Cd("..".to_string())
    )
}

fn special_parse(i: &str) -> IResult<&str, Line> {
    map(
        preceded(
            tag("$ cd "),
            take_while1(|x: char| is_alphabetic(x as u8) || x == '.'),
        ),
        |s: &str| Line::Cd(s.to_string()),
    )(i)
}

fn parse(i: String) -> Line {
    // let cd_parse = map(tag("$ cd "), |s| Line::Cd(s));

    if i.starts_with("$ cd ") {
        return Line::Cd(i.strip_prefix("$ cd ").unwrap().to_string());
    } else if i == "$ ls" {
        return Line::Ls;
    } else if i.starts_with("dir ") {
        return Line::Dir(i.strip_prefix("dir ").unwrap().to_string());
    } else {
        return Line::File(i.split(" ").next().unwrap().parse::<i32>().unwrap());
    }
}

fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let ops = BufReader::new(file)
        .lines()
        // .take(600)
        .filter_map(|x| x.ok())
        .map(parse);

    let mut dirs: HashMap<String, i32> = HashMap::new();
    let mut curr_path = "".to_string();
    let mut c = 0;
    for op in ops.into_iter() {
        c += 1;
        let mut contained_file = false;
        // println!("{:?},{}", op, curr_path);
        match op {
            Line::Cd(v) if v == "/" => curr_path = "".to_string(),
            Line::Cd(v) if v == ".." => {
                curr_path = curr_path[..curr_path.rfind("/").unwrap()].to_string()
            }
            Line::Cd(v) => {
                curr_path += &("/".to_owned() + &v);
                if !contained_file {
                    dirs.insert(curr_path.clone(), 0);
                }
            }
            Line::Ls => {}
            Line::File(n) if dirs.contains_key(&curr_path) => {
                let e = dirs.entry(curr_path.clone()).and_modify(|x| *x += n);
                contained_file = true;
                // println!("{}, {}", curr_path, e.or_default());
            }
            Line::File(n) => {
                dirs.insert(curr_path.clone(), n);
                contained_file = true;
                // println!("{}, {}", curr_path, n);
            }
            Line::Dir(_) => {}
        }
    }
    // println!("count {}", c);

    // println!("{:?}", dirs);
    let mut small_dirs: Vec<(String, i32)> = Vec::new();
    let mut big_dirs: Vec<i64> = Vec::new();
    let req_space: i64 =
        30000000i64 - (70000000i64 - dirs.iter().map(|x| *x.1 as i64).sum::<i64>());

    for (k, v) in dirs.iter() {
        let mut total = 0;
        for (in_k, in_v) in dirs.iter() {
            if in_k.starts_with(k) {
                total += in_v;
            }
        }
        // println!("{:?}", (k.to_owned(), total));
        if total < 100000 {
            small_dirs.push((k.to_owned(), total));
        }

        if total as i64 >= req_space {
            big_dirs.push(total as i64);
        }
    }

    let min_size_dir = big_dirs.iter().min().unwrap();

    // println!("{:?}", small_dirs);
    println!("{}", small_dirs.iter().map(|x| x.1).sum::<i32>());
    println!("{}", min_size_dir);
}
