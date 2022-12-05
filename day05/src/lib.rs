use std::fs;

pub type Input = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

#[inline]
pub fn read_input(filename: &str) -> Input {
    let buffer = fs::read_to_string(filename).unwrap();

    // Find the number of stacks
    let stack_len: usize = buffer
        .lines()
        .find(|line| line.as_bytes()[1].is_ascii_digit())
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|last| last.parse::<usize>().unwrap())
        .unwrap();

    // Parse the stacks
    let stacks: Vec<Vec<char>> = (1..=stack_len * 4)
        .step_by(4)
        .map(|x| {
            buffer
                .lines()
                .filter(|line| !line.as_bytes()[x].is_ascii_whitespace())
                .take_while(|line| !line.as_bytes()[x].is_ascii_digit())
                .map(|line| line.as_bytes()[x] as char)
                .filter(|c| c.is_ascii_uppercase())
                .collect()
        })
        .filter_map(|stack: Vec<char>| {
            if !stack.is_empty() {
                Some(stack.into_iter().rev().collect())
            } else {
                None
            }
        })
        .collect();

    // Parse the moves - Stack# - From - To
    let moves: Vec<(usize, usize, usize)> = buffer
        .lines()
        .filter(|line| line.contains("move"))
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|split: Vec<&str>| {
            (
                split[1].parse::<usize>().unwrap(),
                split[3].parse::<usize>().unwrap() - 1, // - 1 because our idx starts at 0
                split[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    (stacks, moves)
}

#[inline]
pub fn solve_part1(program: &Input) -> String {
    let mut stacks = program.0.clone();
    program.1.iter().for_each(|(many, from, to)| {
        (0..*many).for_each(|_| {
            let v = stacks[*from].pop().unwrap();
            stacks[*to].push(v);
        })
    });
    stacks.iter().map(|s| *s.last().unwrap()).collect()
}

#[inline]
pub fn solve_part2(program: &Input) -> String {
    let mut stacks = program.0.clone();
    program.1.iter().for_each(|(many, from, to)| {
        (0..*many)
            .into_iter()
            .map(|_| stacks[*from].pop().unwrap())
            .collect::<Vec<char>>()
            .iter()
            .rev()
            .for_each(|i| {
                stacks[*to].push(*i);
            });
    });
    stacks.iter().map(|s| *s.last().unwrap()).collect()
}
