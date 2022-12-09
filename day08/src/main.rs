use std::fs::File;
use std::io::{prelude::*, BufReader};

// #[derive(Clone)]
// struct TreeLine(Vec<u8>);

// impl From<Vec<u8>> for TreeLine {
//     fn from(v: Vec<u8>) -> Self {
//         TreeLine(v)
//     }
// }

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn directional_1d_visibilty(tree_line: Vec<u8>) -> Vec<bool> {
    let mut heighest = 0;
    let mut visibility = Vec::new();
    let mut is_first = true;
    for tree in tree_line.iter() {
        if *tree > heighest || is_first {
            visibility.push(true);
        } else {
            visibility.push(false);
        }
        heighest = u8::max(heighest, *tree);
        is_first = false;
    }

    visibility
}

fn two_way_visibility(mut tree_line: Vec<u8>) -> Vec<bool> {
    let visibility = directional_1d_visibilty(tree_line.clone());
    tree_line.reverse();
    let mut rev_vis = directional_1d_visibilty(tree_line);
    rev_vis.reverse();

    visibility
        .iter()
        .zip(rev_vis.iter())
        .map(|(x, y)| x | y)
        .collect()
}

fn visiblity(trees: Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let reg_vis: Vec<Vec<bool>> = trees
        .iter()
        .map(|x| two_way_visibility(x.clone()))
        .collect();
    let t_trees = transpose(trees);
    let t_vis: Vec<Vec<bool>> = t_trees
        .iter()
        .map(|x| two_way_visibility(x.clone()))
        .collect();
    let join_vis: Vec<Vec<bool>> = join_vis(reg_vis, transpose(t_vis));
    join_vis
}

fn join_vis(reg_vis: Vec<Vec<bool>>, t_vis: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    reg_vis
        .iter()
        .zip(t_vis.iter())
        .map(|(l1, l2)| l1.iter().zip(l2.iter()).map(|(x, y)| x | y).collect())
        .collect()
}

fn directional_1d_scenic(tree_line: Vec<u8>) -> Vec<usize> {
    let mut last_tree = [0; 10];
    let scenic_score = tree_line
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let v = i - *last_tree[(*x as usize)..=9usize].iter().max().unwrap();
            last_tree[*x as usize] = i;
            v
        })
        .collect();
    scenic_score
}

fn two_way_score(mut tree_line: Vec<u8>) -> Vec<usize> {
    let score = directional_1d_scenic(tree_line.clone());
    tree_line.reverse();
    let mut rev_score = directional_1d_scenic(tree_line);
    rev_score.reverse();

    score
        .iter()
        .zip(rev_score.iter())
        .map(|(x, y)| x * y)
        .collect()
}

fn scenic_score(trees: Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let reg_vis: Vec<Vec<usize>> = trees.iter().map(|x| two_way_score(x.clone())).collect();
    let t_trees = transpose(trees);
    let t_vis: Vec<Vec<usize>> = t_trees.iter().map(|x| two_way_score(x.clone())).collect();
    let join_vis: Vec<Vec<usize>> = join_score(reg_vis, transpose(t_vis));
    join_vis
}

fn join_score(reg_vis: Vec<Vec<usize>>, t_vis: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    reg_vis
        .iter()
        .zip(t_vis.iter())
        .map(|(l1, l2)| l1.iter().zip(l2.iter()).map(|(x, y)| x * y).collect())
        .collect()
}

fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let trees: Vec<Vec<u8>> = BufReader::new(file)
        .lines()
        // .take(600)
        .filter_map(|x| x.ok())
        .map(|x| {
            x.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .into()
        })
        .collect();

    let vis: u32 = visiblity(trees.clone())
        .iter()
        .map(|x| x.iter().map(|x| *x as u32).sum::<u32>())
        .sum();

    let score: usize = scenic_score(trees)
        .iter()
        .map(|x| *x.iter().max().unwrap())
        .max()
        .unwrap();
    println!("{} trees are visible from the outside", vis);
    println!("{} highest tree scenic view", score);
}

#[test]
fn test_scenic_1d() -> () {
    let trees = Vec::<u8>::from([3, 0, 3, 7, 3]);
    let score = directional_1d_scenic(trees);
    assert_eq!(score, Vec::from([0, 1, 2, 3, 1]))
}

#[test]
fn test_scenic_two_ways() -> () {
    let trees = Vec::<u8>::from([3, 0, 3, 7, 3]);
    let score = two_way_score(trees);
    assert_eq!(score, Vec::from([0, 1, 2, 3, 0]))
}

#[test]
fn test_visibility_1d() -> () {
    let trees = Vec::<u8>::from([3, 0, 3, 7, 3]);

    let vis = two_way_visibility(trees);
    assert_eq!(vis, Vec::from([true, false, false, true, true]))
}

#[test]
fn test_visibility() -> () {
    let trees = Vec::<Vec<u8>>::from([
        Vec::<u8>::from([3u8, 0, 3, 7, 3]),
        Vec::<u8>::from([2, 5, 5, 1, 2]),
        Vec::<u8>::from([6, 5, 3, 3, 2]),
        Vec::<u8>::from([3, 3, 5, 4, 9]),
        Vec::<u8>::from([3, 5, 3, 9, 0]),
    ]);

    let vis = visiblity(trees);
    assert_eq!(
        vis,
        Vec::from([
            [true, true, true, true, true],
            [true, true, true, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, true, true, true, true]
        ])
    )
}
