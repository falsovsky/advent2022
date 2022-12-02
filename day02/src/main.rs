use ::day02::*;
use std::env;

const PART1: u8 = 0b01;
const PART2: u8 = 0b10;

fn solve(program: &[Play], parts: u8) -> (u32, u32) {
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
    let root = env::current_dir().unwrap().display().to_string();
    let code = read_input(format!("{}{}", root, "/input/day02.txt").as_str());
    let (pt1, pt2) = solve(&code, PART1 | PART2);
    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
}

#[cfg(test)]
mod day02 {
    #[test]
    fn part1() {
        use crate::*;
        let root = env::current_dir().unwrap().display().to_string();
        let input = read_input(format!("{}{}", root, "/../input/sample02.txt").as_str());
        assert_eq!(solve_part1(&input), 15);
    }

    #[test]
    fn part2() {
        use crate::*;
        let root = env::current_dir().unwrap().display().to_string();
        let input = read_input(format!("{}{}", root, "/../input/sample02.txt").as_str());
        assert_eq!(solve_part2(&input), 12);
    }
}
