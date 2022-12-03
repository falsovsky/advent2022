use std::fs;

pub type Rucksack = Vec<u8>;

fn get_priority(ascii: char) -> u32 {
    if ('A'..='Z').contains(&ascii) {
        ascii as u32 - 38
    } else if ('a'..='z').contains(&ascii) {
        ascii as u32 - 96
    } else {
        0
    }
}

#[inline]
pub fn read_input(filename: &str) -> Vec<Rucksack> {
    let buffer = fs::read_to_string(filename).unwrap();
    buffer
        .lines()
        .into_iter()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

#[inline]
pub fn solve_part1(program: &[Rucksack]) -> u32 {
    program
        .iter()
        .map(|rucksack| {
            let (h1, h2) = (
                rucksack[..rucksack.len() / 2].to_vec(),
                rucksack[rucksack.len() / 2..].to_vec(),
            );
            get_priority(
                h1.iter()
                    .find(|f| h2.contains(f))
                    .map(|&f| f as char)
                    .unwrap(),
            )
        })
        .sum()
}

#[inline]
pub fn solve_part2(program: &[Rucksack]) -> u32 {
    program
        .chunks(3)
        .map(|chunk| {
            let (a, b, c) = (
                chunk.get(0).unwrap(),
                chunk.get(1).unwrap(),
                chunk.get(2).unwrap(),
            );
            get_priority(
                a.iter()
                    .find(|f| b.contains(f) && c.contains(f))
                    .map(|&f| f as char)
                    .unwrap(),
            )
        })
        .sum()
}
