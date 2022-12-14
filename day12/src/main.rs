use pathfinding::prelude::astar;

use std::cmp::max;
use std::fs::File;

use std::io::{prelude::*, BufReader};
const COL_LENGTH: usize = 173;
const ROW_LENGTH: usize = 41;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Tile {
    x: usize,
    y: usize,
    height: Option<u32>,
}

// #[test]
// fn test_reduce_usize() -> () {
//     let x = 2usize - 1;
//     assert_eq!(0usize - x, 2)
// }

impl Tile {
    fn is_valid(&self) -> bool {
        self.x < COL_LENGTH && self.y < ROW_LENGTH
    }
    fn successors<'a>(&self, map: &[[u32; COL_LENGTH]; ROW_LENGTH]) -> Vec<(Tile, u32)> {
        // dbg!(self);
        [
            Tile {
                x: self.x,
                y: self.y.checked_sub(1).unwrap_or(ROW_LENGTH + 1),
                height: self.height(map),
            },
            Tile {
                x: self.x + 1,
                y: self.y,
                height: self.height(map),
            },
            Tile {
                x: self.x.checked_sub(1).unwrap_or(COL_LENGTH + 1),
                y: self.y,
                height: self.height(map),
            },
            Tile {
                x: self.x,
                y: self.y + 1,
                height: self.height(map),
            },
        ]
        .iter()
        .filter(|x| {
            x.is_valid()
                && x.height
                    .unwrap_or(27)
                    .checked_sub(self.height.unwrap_or(27))
                    .unwrap_or(0)
                    <= 1
        })
        .map(|x| (x.clone(), 1u32))
        .collect::<Vec<(Tile, u32)>>()
    }

    fn height(&self, map: &[[u32; COL_LENGTH]; ROW_LENGTH]) -> Option<u32> {
        match map[self.y][self.x] {
            69 => None,
            83 => Some(1),
            v => Some(v - 96),
        }
    }
}

fn distance(t1: &Tile, t2: &Tile) -> u32 {
    max(
        (t1.x.abs_diff(t2.x) + t1.y.abs_diff(t2.y))
            .try_into()
            .unwrap(),
        t1.height.unwrap_or(27).abs_diff(t2.height.unwrap_or(27)),
    )
}
fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let map = load_map(file);
    let start = find_start(&map);
    let end = find_end(&map);
    let a_result = astar(
        &start,
        |t| Tile::successors(t, &map),
        |t| distance(t, &end),
        |t| t.height.is_none(),
    )
    // .unwrap()
    ;
    println!("{:?}", a_result.unwrap().0.len());

    let bees = (0..ROW_LENGTH).map(|x| (x, 1));
    let results = bees.map(|(y, x)| {
        astar(
            &Tile {
                x,
                y,
                height: Some(2),
            },
            |t| Tile::successors(t, &map),
            |t| distance(t, &end),
            |t| t.height.is_none(),
        )
    });
    println!(
        "{:?}",
        results.into_iter().map(|x| x.unwrap().0.len()).min()
    );
}

fn find_end(_: &[[u32; COL_LENGTH]; ROW_LENGTH]) -> Tile {
    Tile {
        x: 149,
        y: 20,
        height: None,
    }
}

fn find_start(_: &[[u32; COL_LENGTH]; ROW_LENGTH]) -> Tile {
    Tile {
        x: 1,
        y: 20,
        height: Some(1),
    }
}

fn load_map(file: File) -> [[u32; COL_LENGTH]; ROW_LENGTH] {
    let map = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|l| {
            l.chars()
                .filter_map(|x| {
                    if !x.is_ascii_control() {
                        return Some(x as u32);
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<u32>>()
        });
    let mut arr = [[0u32; COL_LENGTH]; ROW_LENGTH];
    for (i, line) in map.into_iter().enumerate() {
        for (j, num) in line.into_iter().enumerate() {
            arr[i][j] = num;
        }
    }
    arr
}
