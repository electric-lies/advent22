use itertools::Itertools;
use json::{array, parse, JsonValue};
use std::fs::File;
use std::io::{prelude::*, BufReader};
// let parsed = json::parse()

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet(JsonValue);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match compare(&self.0, &other.0) {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Equal,
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match compare(&self.0, &other.0) {
            Some(true) => std::cmp::Ordering::Greater,
            Some(false) => std::cmp::Ordering::Less,
            None => std::cmp::Ordering::Equal,
        }
    }
}

fn compare(j1: &JsonValue, j2: &JsonValue) -> Option<bool> {
    match (j1, j2) {
        (JsonValue::Number(v1), JsonValue::Number(v2)) => {
            if v1.as_fixed_point_i64(2).unwrap() > v2.as_fixed_point_i64(2).unwrap() {
                return Some(false);
            } else if v1.eq(v2) {
                return None;
            } else {
                return Some(true);
            }
        }

        (JsonValue::Array(a1), JsonValue::Array(a2)) => return compare_arr(a1, a2),
        (JsonValue::Array(a), JsonValue::Number(v)) => {
            // println!("arr num");
            return compare_arr(a, &Vec::<JsonValue>::from([JsonValue::Number(*v)]));
        }
        (JsonValue::Number(v), JsonValue::Array(a)) => {
            // println!("num arr");
            return compare_arr(&Vec::<JsonValue>::from([JsonValue::Number(*v)]), a);
        }
        _ => panic!("unexpected json"),
    }
}

fn compare_arr(a1: &Vec<JsonValue>, a2: &Vec<JsonValue>) -> Option<bool> {
    for (e1, e2) in a1.iter().zip(a2.iter()) {
        if let Some(v) = compare(e1, e2) {
            return Some(v);
        }
    }
    if a1.len() < a2.len() {
        return Some(true);
    } else if a1.len() == a2.len() {
        return None;
    } else {
        return Some(false);
    }
}

fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let packet_tups = BufReader::new(file)
        .lines()
        // .take(2)
        .filter_map(|x| x.ok())
        .map(|x| parse(&x))
        .tuples()
        .map(|(p1, p2, _)| {
            let x = compare(&p1.unwrap(), &p2.unwrap());
            // println!("{:?}", x);
            x
        });

    println!(
        "part one: {:?}",
        packet_tups
            .enumerate()
            .map(|(i, x)| if x == Some(true) { i + 1 } else { 0 })
            .sum::<usize>()
    );

    let file = File::open("input").expect("did not found file");
    let mut packets = BufReader::new(file)
        .lines()
        // .take(2)
        .filter_map(|x| x.ok())
        .filter(|x| x != "" && x != " ")
        .map(|x| Packet(parse(&x).unwrap()))
        .collect_vec();
    let first_div = Packet(array![array![JsonValue::Number(2.into())]]);
    let second_div = Packet(array![array![JsonValue::Number(6.into())]]);
    packets.push(first_div.clone());
    packets.push(second_div.clone());

    println!("-----");

    packets.sort();

    // for p in packets {
    //     println!("{}", p.0);
    // }

    let result_2 = packets
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if *x == first_div || *x == second_div {
                i + 1
            } else {
                1
            }
        })
        .fold(1, |acc, e| acc * e);
    println!("second result: {:?}", result_2);
}
