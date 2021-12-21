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

const LINE_LENGTH: usize = 12;

fn part1() {
    if let Ok(lines) = read_lines("input") {
        let mut counts = [0; LINE_LENGTH];
        for line in lines {
            let line = line.unwrap();
            for (idx, &b) in line.as_bytes().iter().enumerate() {
                counts[idx] += if b == b'1' { 1 } else { -1 };
            }
        }

        let mut gamma: usize = 0;
        for (idx, count) in counts.into_iter().rev().enumerate() {
            // Bug: was constructing number in reverse order, fixed with .rev()
            gamma |= ((count >= 0) as usize) << idx;
        }

        let epsilon = !gamma & (!0 >> (usize::BITS as usize - LINE_LENGTH));
        // println!("gamma: {:#032b}, epsilon: {:#032b}", gamma, epsilon);
        println!("gamma: {}, epsilon: {}", gamma, epsilon);
        println!("final answer: {}", gamma * epsilon);
    }
}

fn part2() {
    if let Ok(lines) = read_lines("input") {
        let mut counts = [0; LINE_LENGTH];
        let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();

        for line in lines.iter().map(|elt| elt.as_str()) {
            for (idx, &b) in line.as_bytes().iter().enumerate() {
                counts[idx] += if b == b'1' { 1 } else { -1 };
            }
        }

        fn find_rating<F>(lines: &Vec<String>, test: F) -> usize
        where
            F: Fn(i32) -> bool,
        {
            let mut bit = 0;
            let mut valid: HashSet<usize> = (0..lines.len()).collect();
            while valid.len() > 1 {
                let mut counts = [0; LINE_LENGTH];
                for &idx in valid.iter() {
                    for (idx, &b) in lines[idx].as_bytes().iter().enumerate() {
                        counts[idx] += if b == b'1' { 1 } else { -1 };
                    }
                }

                let check: u8 = if test(counts[bit]) { b'1' } else { b'0' };
                let valid_copy = valid.clone();
                // println!(
                //     "{:?}",
                //     valid
                //         .iter()
                //         .map(|&e| &lines[e])
                //         .collect::<HashSet<&String>>()
                // );
                // println!("{:?}", counts);
                for (idx, line) in lines
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| valid_copy.contains(&i))
                {
                    // Bug: wasn't recalclating bit counts as set of numbers shrinks
                    // println!(
                    //     "check: {}, line: {}, checking bit: {}",
                    //     check as char,
                    //     line,
                    //     line.as_bytes()[bit] as char
                    // );
                    if line.as_bytes()[bit] != check {
                        // println!(
                        //     "removing {} ({})",
                        //     line,
                        //     usize::from_str_radix(line, 2).unwrap()
                        // );
                        valid.remove(&idx);
                        if valid.len() == 1 {
                            break;
                        }
                    }
                }
                bit += 1;
            }
            usize::from_str_radix(&lines[*valid.iter().next().unwrap()], 2).unwrap()
        }

        // find oxygen generator rating
        let oxygen = find_rating(&lines, |count| count >= 0);
        let co2 = find_rating(&lines, |count| count < 0);
        println!("oxygen generator rating: {}", oxygen);
        println!("CO2 scrubber rating: {}", co2);
        println!("final answer: {}", oxygen * co2)
    }
}

fn main() {
    part1();
    part2();
}
