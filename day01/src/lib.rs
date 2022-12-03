use std::fs;
use std::str::FromStr;


#[inline]
pub fn read_input(filename: &str) -> Vec<u32> {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut input: Vec<u32> = Vec::new();
    let mut total: u32 = 0;
    buffer.lines().for_each(|line| {
        if !line.is_empty() {
            let value = u32::from_str(line).unwrap();
            if let Some(sum) = total.checked_add(value) {
                total = sum;
            }
        } else {
            input.push(total);
            total = 0;
        };
    });
    if total > 0 {
        input.push(total);
    }
    input.sort_unstable();
    input
}

#[inline]
pub fn solve_part1(program: &[u32]) -> u32 {
    *program.last().unwrap()
}

#[inline]
pub fn solve_part2(program: &[u32]) -> u32 {
    program.iter().rev().take(3).sum()
}