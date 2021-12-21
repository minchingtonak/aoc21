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

fn part1() {
    if let Ok(lines) = read_lines("input") {
        let mut hor_pos: u32 = 0;
        let mut depth: u32 = 0;
        for line in lines {
            let text = line.unwrap();
            let line: Vec<&str> = text.split_ascii_whitespace().collect();
            match line.as_slice() {
                [cmd, n] => match (*cmd, n.parse::<u32>().unwrap()) {
                    ("forward", n) => hor_pos += n,
                    ("down", n) => depth += n,
                    ("up", n) => depth -= n,
                    _ => {}
                },
                _ => {}
            }
        }

        println!("horizontal pos: {}, depth: {}", hor_pos, depth);
        println!("final answer: {}", hor_pos * depth as u32);
    }
}

fn part2() {
    if let Ok(lines) = read_lines("input") {
        let mut hor_pos: u32 = 0;
        let mut aim: u32 = 0;
        let mut depth: u32 = 0;
        for line in lines {
            let text = line.unwrap();
            let line: Vec<&str> = text.split_ascii_whitespace().collect();
            match line.as_slice() {
                [cmd, n] => match (*cmd, n.parse::<u32>().unwrap()) {
                    ("forward", n) => {
                        hor_pos += n;
                        depth += n * aim;
                    }
                    ("down", n) => aim += n,
                    ("up", n) => aim -= n,
                    _ => {}
                },
                _ => {}
            }
        }

        println!("horizontal pos: {}, depth: {}, aim: {}", hor_pos, depth, aim);
        println!("final answer: {}", hor_pos * depth);
    }
}

fn main() {
    part1();
    part2();
}
