use std::fs::File;
use std::io::{prelude::*, BufReader};

fn locate(window_size: usize, buf: &[u8]) -> usize {
    buf.windows(window_size)
        .zip(window_size..)
        .find_map(|(window, i)| if all_different(window) { Some(i) } else { None })
        .unwrap()
}

fn main() {
    let file = File::open("input").expect("did not found file");
    let mut buf = [0u8; 4096];
    BufReader::new(file).read(&mut buf).expect("read failed");
    let place1 = locate(4, &buf);
    let place2 = locate(14, &buf);

    println!("1: {}\n2: {}", place1, place2);
}

fn all_different(window: &[u8]) -> bool {
    let mut set = window.to_vec();
    set.sort();
    set.dedup();
    set.len() == window.len()
}
