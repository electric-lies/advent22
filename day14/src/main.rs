use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let lines = BufReader::new(file)
        .lines()
        // .take(2)
        .filter_map(|x| x.ok())
        .map(|x| parse(&x))
        .collect::<Vec<Line>>();
    println!("{:?}", lines);

    let mut map = HashSet::<Point>::new();
    let mut safe_cols = HashSet::<usize>::new();
    let mut max_y = 0;
    for line in lines {
        let mut line_iter = line.iter();
        let mut base_point = line_iter.next().expect("empty line");

        map.insert(*base_point);
        safe_cols.insert(base_point.x);
        max_y = max(max_y, base_point.y);
        for point in line_iter {
            safe_cols.insert(point.x);
            max_y = max(max_y, point.y);
            if point.x == base_point.x {
                for i in min(base_point.y, point.y)..=max(base_point.y, point.y) {
                    map.insert(Point { x: point.x, y: i });
                    max_y = max(max_y, i);
                }
            } else if point.y == base_point.y {
                for i in min(base_point.x, point.x)..=max(base_point.x, point.x) {
                    safe_cols.insert(i);
                    map.insert(Point { x: i, y: point.y });
                }
            } else {
                panic!("diagonal line");
            }
            base_point = point;
        }
    }

    let base_map = map.clone();
    let rest_counter = simulate(
        &|x: usize, y: usize, _| !safe_cols.contains(&x) || y == max_y,
        map,
        -1,
    );

    let map = base_map;

    let second_rest_counter = simulate(
        &|x: usize, y: usize, m| m.contains(&Point { x: 500, y: 0 }),
        map,
        (max_y + 1) as i32,
    );

    println!("part one:{}", rest_counter);
    println!("part two:{}", second_rest_counter);

    // println!("{:?}", rest_counter);
}

fn simulate(
    termination_cond: &dyn Fn(usize, usize, &HashSet<Point>) -> bool,
    mut map: HashSet<Point>,
    floor: i32,
) -> i32 {
    let mut sim_end = false;
    let mut rest_counter = 0;
    loop {
        let mut sand_x = 500usize;
        let mut sand_y = 0usize;
        loop {
            if termination_cond(sand_x, sand_y, &map) {
                sim_end = true;
                break;
            }
            if sand_y as i32 == floor {
                map.insert(Point {
                    x: sand_x,
                    y: sand_y,
                });
                break;
            }
            if !map.contains(&Point {
                x: sand_x,
                y: sand_y + 1,
            }) {
                sand_y += 1;
                continue;
            } else if !map.contains(&Point {
                x: sand_x - 1,
                y: sand_y + 1,
            }) {
                sand_x -= 1;
                sand_y += 1;
                continue;
            } else if !map.contains(&Point {
                x: sand_x + 1,
                y: sand_y + 1,
            }) {
                sand_x += 1;
                sand_y += 1;
                continue;
            } else {
                map.insert(Point {
                    x: sand_x,
                    y: sand_y,
                });
                break;
            }
        }
        if sim_end {
            break;
        }
        rest_counter += 1;
        // println!("{}", rest_counter);
    }
    rest_counter
}

type Line = Vec<Point>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn parse(i: &str) -> Line {
    i.split(" -> ")
        .map(|p| {
            let tup = p.split_once(",").unwrap();
            Point {
                x: tup.0.parse().unwrap(),
                y: tup.1.parse().unwrap(),
            }
        })
        .collect()
    // Default::default()
}
