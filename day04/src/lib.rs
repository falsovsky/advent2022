use std::fs;

pub type Assignment = (Vec<u32>, Vec<u32>);

#[inline]
pub fn read_input(filename: &str) -> Vec<Assignment> {
    let buffer = fs::read_to_string(filename).unwrap();
    buffer
        .lines()
        .into_iter()
        .map(|line| -> (Vec<u32>, Vec<u32>) {
            let items = line.split(',').collect::<Vec<&str>>();
            let a = items[0].split('-').collect::<Vec<&str>>();
            let b = items[1].split('-').collect::<Vec<&str>>();
            (
                (a[0].parse().unwrap()..=a[1].parse().unwrap()).collect(),
                (b[0].parse().unwrap()..=b[1].parse().unwrap()).collect(),
            )
        })
        .collect()
}

#[inline]
pub fn solve_part1(program: &[Assignment]) -> u32 {
    program
        .iter()
        .filter(|(h1, h2)| {
            let h1_items: Vec<u32> = h1.into_iter().cloned().collect();
            let h2_items: Vec<u32> = h2.into_iter().cloned().collect();
            h1_items.iter().all(|item| h2_items.contains(item))
                || h2_items.iter().all(|item| h1_items.contains(item))
        })
        .count() as u32
}

#[inline]
pub fn solve_part2(program: &[Assignment]) -> u32 {
    program
        .iter()
        .filter(|(h1, h2)| h1.into_iter().any(|x| h2.contains(x)))
        .count() as u32
}
