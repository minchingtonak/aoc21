use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const BOARD_SIZE: usize = 5;
struct Board {
    spaces: [u32; BOARD_SIZE],
}

impl Board {
    fn new(lines: io::Lines<io::BufReader<File>>) -> Board {
        let mut b = Board {
            spaces: [0; BOARD_SIZE],
        };

        for line in lines.map(|e| e.unwrap()) {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .enumerate()
                .for_each(|(idx, n)| b.spaces[idx] = n);
        }

        b
    }
}

fn part1() {
    let mut lines = read_lines("input").expect("failed to open input file");

    let chosen: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    for line in lines.map(|e| e.unwrap()) {

    }
}

fn part2() {}

fn main() {
    part1();
    part2();
}
