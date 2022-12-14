use itertools::Itertools;

use std::fs::File;
use std::io::{prelude::*, BufReader};
#[derive(Debug, Clone)]
enum Operation {
    Mul,
    Add,
}
#[derive(Debug, Clone)]
enum Warriness {
    Num(i128),
    Old,
}

#[derive(Debug, Clone)]
struct Monkey {
    num: usize,
    items: Vec<i128>,
    operation: (Operation, Warriness),
    div_test_value: i128,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_monkey(l1: String, l2: String, l3: String, l4: String, l5: String, l6: String) -> Monkey {
    println!("1");
    let monkey_num = l1
        .strip_prefix("Monkey ")
        .expect("first line does not start with 'Monkey '")
        .trim_end_matches(":")
        .parse::<usize>()
        .expect("failed to parse monkey num");

    let mut items = l2
        .strip_prefix("  Starting items: ")
        .expect("second line dont starts with 'Starting items: '")
        .split(", ")
        .map(|s| s.parse::<i128>().expect("item parse failed"))
        .collect::<Vec<i128>>();
    items.reverse();
    let operation: (&str, &str) = l3
        .strip_prefix("  Operation: new = old ")
        .expect("Third line does not start with ...")
        .split(" ")
        .collect_tuple()
        .expect("too many values");

    let operation_sign = match operation.0 {
        "*" => Operation::Mul,
        "+" => Operation::Add,
        _ => panic!("forbidden op found during parsing"),
    };
    let operation_value = match operation.1 {
        "old" => Warriness::Old,
        n => Warriness::Num(n.parse::<i128>().unwrap()),
    };

    let test = l4
        .split(" ")
        .last()
        .expect("Fourth line parse failed")
        .parse::<i128>()
        .unwrap();

    let true_monkey = l5
        .split(" ")
        .last()
        .expect("Fifth line parse failed")
        .parse::<usize>()
        .unwrap();

    let false_monkey = l6
        .split(" ")
        .last()
        .expect("sixth line parse failed")
        .parse::<usize>()
        .unwrap();
    Monkey {
        num: monkey_num,
        items,
        operation: (operation_sign, operation_value),
        div_test_value: test,
        true_monkey,
        false_monkey,
    }
}

#[test]
fn test_div() -> () {
    assert_eq!(4 / 3, 1)
}
fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let mut monkys = BufReader::new(file)
        .lines()
        // .take(600)
        .filter_map(|x| x.ok())
        .tuples()
        .map(|(l1, l2, l3, l4, l5, l6, _)| parse_monkey(l1, l2, l3, l4, l5, l6))
        .collect::<Vec<Monkey>>();

    let m = monkys
        .iter()
        .map(|x| x.div_test_value)
        .fold(1, |acc, x| acc * x);
    println!("{m}");

    let mut items: [Vec<i128>; 8] = Default::default();
    let mut inspections = [0u32; 8];
    for mon in monkys.clone() {
        items[mon.num] = mon.items;
    }

    for _ in 0..10_000 {
        monkys.sort_by_key(|x| x.num);
        for monkey in monkys.iter_mut() {
            for _ in 0..items[monkey.num].len() {
                let item = items[monkey.num].pop().unwrap();
                let v = match monkey.operation.1 {
                    Warriness::Num(n) => n,
                    Warriness::Old => item,
                };
                inspections[monkey.num] += 1;
                let tmp_item = match monkey.operation.0 {
                    Operation::Mul => item * v,
                    Operation::Add => item + v,
                } % m;

                if tmp_item % monkey.div_test_value == 0 {
                    items[monkey.true_monkey].push(tmp_item);
                } else {
                    items[monkey.false_monkey].push(tmp_item);
                }
            }
        }
    }

    inspections.sort();
    println!("{:?}", inspections)
}
