use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[inline]
pub fn read_input(filename: &str) -> Vec<u32> {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<u32> = Vec::new();
    let mut total: u32 = 0;
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        if let Ok(value) = line_str.parse::<u32>() {
            if let Some(sum) = total.checked_add(value) {
                total = sum;
            }
        } else {
            input.push(total);
            total = 0;
        };
    }
    if total > 0 {
        input.push(total);
    }
    input.sort();
    input
}

#[inline]
pub fn solve_part1(program: &[u32]) -> u32 {
    let mut result: u32 = 0;
    if let Some(value) = program.last() {
        result = *value;
    }
    result
}

#[inline]
pub fn solve_part2(program: &[u32]) -> u32 {
    program.iter().rev().take(3).sum()
}