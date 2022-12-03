use std::fs;
use std::cmp::Ordering;


#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other { return Ordering::Equal }
        match (self, other) {
            (Move::Rock, Move::Paper) => Ordering::Less,
            (Move::Paper, Move::Scissors) => Ordering::Less,
            (Move::Scissors, Move::Rock) => Ordering::Less,
            (_, _) => Ordering::Greater
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Clone, Copy)]
pub struct Play {
    player1: Move,
    player2: Move,
    status: Status,
}

impl Play {
    pub fn get_score(&self) -> u32 {
        match self.player2.cmp(&self.player1) {
            Ordering::Equal => self.player2 as u32 + Status::Draw as u32, // Draw
            Ordering::Greater => self.player2 as u32 + Status::Win as u32, // Win
            Ordering::Less => self.player2 as u32 + Status::Lose as u32 // Lose
        }
    }

    pub fn get_real_score(&self) -> u32 {
        match (self.status, self.player1) {
            (Status::Draw, player1) => player1 as u32 + self.status as u32,
            (Status::Lose, Move::Paper) => Move::Rock as u32 + self.status as u32,
            (Status::Lose, Move::Rock) => Move::Scissors as u32 + self.status as u32,
            (Status::Lose, Move::Scissors) => Move::Paper as u32 + self.status as u32,
            (Status::Win, Move::Rock) => Move::Paper as u32 + self.status as u32,
            (Status::Win, Move::Paper) => Move::Scissors as u32 + self.status as u32,
            (Status::Win, Move::Scissors) => Move::Rock as u32 + self.status as u32,
        }
    }
}

#[inline]
pub fn read_input(filename: &str) -> Vec<Play> {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut plays: Vec<Play> = Vec::new();
    const PLAY_MAP: &[((char, char), Play)] = &[
        (('A', 'X'), Play { player1: Move::Rock, player2: Move::Rock, status: Status::Lose }),
        (('A', 'Y'), Play { player1: Move::Rock, player2: Move::Paper, status: Status::Draw }),
        (('A', 'Z'), Play { player1: Move::Rock, player2: Move::Scissors, status: Status::Win }),
        (('B', 'X'), Play { player1: Move::Paper, player2: Move::Rock, status: Status::Lose }),
        (('B', 'Y'), Play { player1: Move::Paper, player2: Move::Paper, status: Status::Draw }),
        (('B', 'Z'), Play { player1: Move::Paper, player2: Move::Scissors, status: Status::Win }),
        (('C', 'X'), Play { player1: Move::Scissors, player2: Move::Rock, status: Status::Lose }),
        (('C', 'Y'), Play { player1: Move::Scissors, player2: Move::Paper, status: Status::Draw }),
        (('C', 'Z'), Play { player1: Move::Scissors, player2: Move::Scissors, status: Status::Win }),
    ];
    buffer.lines().for_each(|line| {
        let line_bytes = line.as_bytes();
        let (player1, player2) = (line_bytes[0] as char, line_bytes[2] as char);
        let play = PLAY_MAP
            .iter()
            .find(|((p1, p2), _)| p1 == &player1 && p2 == &player2)
            .map(|(_, play)| play)
            .unwrap();
        plays.push(*play);
    });
    plays
}

#[inline]
pub fn solve_part1(program: &[Play]) -> u32 {
    program
        .iter()
        .map(Play::get_score)
        .sum()
}

#[inline]
pub fn solve_part2(program: &[Play]) -> u32 {
    program
        .iter()
        .map(Play::get_real_score)
        .sum()
}
