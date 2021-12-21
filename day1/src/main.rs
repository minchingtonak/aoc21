use std::collections::VecDeque;
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
    if let Ok(mut lines) = read_lines("input") {
        let mut count: u32 = 0;
        let mut prev: u32 = lines.next().unwrap().unwrap().parse().unwrap();
        for line in lines {
            if let Ok(curr) = line.unwrap().parse::<u32>() {
                if curr > prev {
                    count += 1;
                }
                prev = curr;
            }
        }

        println!("{} increasing lines counted", count);
    }
}

fn part2() {
    if let Ok(mut lines) = read_lines("input") {
        let mut first: VecDeque<u32> = VecDeque::new();
        let mut second: VecDeque<u32> = VecDeque::new();
        for i in 0..4 {
            let n = lines.next().unwrap().unwrap().parse().unwrap();
            if i > 0 {
                first.push_back(n);
            }
            if i < 3 {
                second.push_back(n);
            }
        }

        println!("{:?} {:?}", first, second);
        
        let mut count: u32 = (first.iter().sum::<u32>() > second.iter().sum::<u32>()) as u32;
        
        for line in lines {
            println!("{:?} {:?}", first, second);
            if let Ok(n) = line.unwrap().parse::<u32>() {
                second.pop_front();
                second.push_back(*first.back().unwrap());
                first.pop_front();
                first.push_back(n);

                if first.iter().sum::<u32>() > second.iter().sum::<u32>() {
                    count += 1;
                }
            }
        }

        println!("{} increasing lines counted", count);
    }
}

fn main() {
    part1();
    part2();
}
