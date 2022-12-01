use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

fn read_input() -> Vec<u32> {
    let filename = "input/day01.txt";
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

fn solve_part1(program: &[u32]) -> u32 {
    program[program.len() - 1]
}

fn solve_part2(program: &[u32]) -> u32 {
    let mut top3: u32 = 0;
    for value in program.iter().rev().take(3) {
        if let Some(sum) = top3.checked_add(*value) {
            top3 = sum;
        }
    }
    top3
}

fn solve(program: &[u32], parts: u8) -> (u32, u32) {
    let runpt1: bool = parts & PART1 != 0;
    let runpt2: bool = parts & PART2 != 0;
    let mut pt1 = 0;
    let mut pt2 = 0;
    if runpt1 {
        pt1 = solve_part1(program);
    }
    if runpt2 {
        pt2 = solve_part2(program);
    }
    (pt1, pt2)
}

fn main() {
    let code = read_input();
    let (pt1, pt2) = solve(&code, PART1 | PART2);
    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
}