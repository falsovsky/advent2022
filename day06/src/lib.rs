use std::fs;

#[inline]
pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn is_unique(vec: &[char]) -> bool {
    vec.iter()
        .all(|ch| vec.iter().filter(|c| *c == ch).count() == 1)
}

#[inline]
pub fn solve_part1(program: &str) -> u32 {
    program
        .chars()
        .enumerate()
        .map(|(idx, _)| (idx + 1) as u32)
        .find(|&idx| {
            if idx >= 4 {
                let start = (idx - 4) as usize;
                let end = idx as usize;
                let tmp: Vec<char> = program[start..end].chars().collect();
                return is_unique(&tmp);
            }
            false
        })
        .unwrap_or(0)
}

#[inline]
pub fn solve_part2(program: &str) -> u32 {
    program
        .chars()
        .enumerate()
        .map(|(idx, _)| (idx + 1) as u32)
        .find(|&idx| {
            if idx >= 14 {
                let start = (idx - 4) as usize;
                let end = idx as usize;
                let tmp: Vec<char> = program[start..end].chars().collect();
                return is_unique(&tmp);
            }
            false
        })
        .unwrap_or(0)
}
