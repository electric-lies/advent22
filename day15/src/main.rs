use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Circle {
    center: Point,
    radius: i32,
}
impl Circle {
    fn contains(&self, point: Point) -> bool {
        distance(self.center, point) <= self.radius
    }
}

fn distance(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn point(i: &str) -> Point {
    //x=2, y=18
    let (x, y) = i.split_once(", ").expect("bad point formatting");
    Point {
        x: x.strip_prefix("x=").unwrap().parse().unwrap(),
        y: y.strip_prefix("y=").unwrap().parse().unwrap(),
    }
}

fn circle(i: &str) -> (Circle, Point) {
    let (sensor_input, beacon_input) = i
        .split_once(": closest beacon is at ")
        .expect("bad input formating");
    let sensor = point(
        sensor_input
            .strip_prefix("Sensor at ")
            .expect("bad input fotrmatting"),
    );
    let beacon = point(beacon_input);
    (
        Circle {
            center: sensor,
            radius: distance(sensor, beacon),
        },
        beacon,
    )
}

fn intersection(c: &Circle, line: i32) -> Option<Segment> {
    if line >= (c.center.y - c.radius) && line <= (c.center.y + c.radius) {
        Some(Segment {
            start: max(c.center.x - c.radius + line.abs_diff(c.center.y) as i32, 0),
            end: min(
                c.center.x + c.radius - line.abs_diff(c.center.y) as i32,
                MAX_LINES,
            ),
        })
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Segment {
    start: i32,
    end: i32,
}

const LINE: i32 = 2_000_000;
// const MAX_LINES: i32 = 20;
const MAX_LINES: i32 = 4_000_000;

fn main() {
    println!("Hello, world!");
    let mut beacons = HashSet::<Point>::new();
    let file = File::open("input").expect("did not found file");
    let circles = BufReader::new(file)
        .lines()
        // .take(2)
        .filter_map(|x| x.ok())
        .map(|x| {
            let (circle, beacon) = circle(&x);
            beacons.insert(beacon);
            circle
        })
        .collect::<Vec<Circle>>();

    // println!("{:?}", circles);

    let max_x = circles
        .clone()
        .iter()
        .map(|c| c.center.x + c.radius)
        .max()
        .unwrap();

    let min_x = circles
        .clone()
        .iter()
        .map(|c| c.center.x - c.radius)
        .min()
        .unwrap();
    println!("line {} from {} to {}", LINE, min_x, max_x);
    let part_one_counter: i32 = (min_x..=max_x)
        .map(|i| {
            let curr_point = Point { x: i, y: LINE };
            if !beacons.contains(&curr_point) && circles.iter().any(|c| c.contains(curr_point)) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("part one:{}", part_one_counter);

    for line in 0..MAX_LINES {
        if line % 10000 == 0 {
            println!("processing line {}", line)
        }
        let mut segments = circles
            .iter()
            .map(|c| intersection(c, line))
            .filter_map(|x| x.ok_or(()).ok())
            .collect::<Vec<Segment>>();
        segments.sort_by_key(|s| s.start);
        let seg = contigiuas_line(segments);
        if seg
            != (Segment {
                start: 0,
                end: MAX_LINES,
            })
        {
            println!("part two: {} => {:?}", line, seg);
            break;
        }
        // println!("--------------------------")
    }
}

fn contigiuas_line(segments: Vec<Segment>) -> Segment {
    let mut segments = segments.iter();
    let first_segment = segments.next().expect("empty line");
    if first_segment.start != 0 {
        return first_segment.clone();
    };
    let mut curr_end = first_segment.end;
    for seg in segments {
        if seg.start <= curr_end + 1 {
            // println!("passed segment: {:?}", seg);
            curr_end = max(curr_end, seg.end);
        } else {
            // println!("curr segment: {:?}", seg);
            break;
        }
    }
    Segment {
        start: 0,
        end: curr_end,
    }
}
