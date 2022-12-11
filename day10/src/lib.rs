use core::panic;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Op {
    Nop,
    AddX(i32)
} 

pub type Input = Vec<Op>;

#[inline]
pub fn read_input(filename: &str) -> Input {
    let buffer = fs::read_to_string(filename).unwrap();
    let lines = buffer.lines().collect::<Vec<&str>>();
    let mut operations: Input = Vec::new();
    for line in lines {
        let s = line.split_whitespace().collect::<Vec<&str>>();
        let op = match s[0] {
            "noop" => Op::Nop,
            "addx" => Op::AddX(s[1].parse().unwrap()),
            _ => panic!("Invalid operation"),
        };
        operations.push(op);
    }
    operations
}

#[inline]
pub fn solve_part1(program: &Input) -> i32 {
    let mut cycles: HashMap<usize, i32> = HashMap::new();
    let mut x = 1;
    let mut cycle: usize = 0;
    for op in program {
        match op {
            Op::Nop => {
                cycle += 1;
                cycles.insert(cycle, x);
            }
            Op::AddX(v) => {
                cycle += 1;
                cycles.insert(cycle, x);

                cycle += 1;
                cycles.insert(cycle, x);                
                x += v;
            }
        }
    }

    let mut total: usize = 0;
    let checks = [20, 60, 100, 140, 180, 220];
    for c in checks {
        total += c * *cycles.get(&c).unwrap() as usize;
    }
    total as i32
}

fn print_crt(x: i32, cycle: i32) -> String {
    let mut s: String = String::new();

    let v = vec![x, x + 1, x + 2];

    let mut c = cycle.rem_euclid(40);
    if c == 0 { c = 40 }

    if v.contains(&c) {
        s = [s, String::from("#")].concat();
    } else {
        s = [s, String::from(".")].concat();
    }

    if c == 40 {
        s = [s, String::from("\n")].concat();
    }
    s
}

#[inline]
pub fn solve_part2(program: &Input) -> String {
    let mut result: String = String::new();

    let mut x = 1;
    let mut cycle: usize = 0;

    for op in program {
        match op {
            Op::Nop => {
                cycle += 1;
                result = [result, print_crt(x, cycle as i32)].concat();
            }
            Op::AddX(v) => {
                cycle += 1;
                result = [result, print_crt(x, cycle as i32)].concat();
                cycle += 1;
                result = [result, print_crt(x, cycle as i32)].concat();
                x += v;
            }
        }
    }
    result
}
