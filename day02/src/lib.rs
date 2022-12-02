use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
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
        (match self.status {
            Status::Draw => self.player1,
            Status::Lose => {
                match self.player1 {
                    Move::Paper => Move::Rock,
                    Move::Rock => Move::Scissors,
                    Move::Scissors => Move::Paper
                }
            },
            Status::Win => {
                match self.player1 {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock
                }
            }
        }) as u32 + self.status as u32
    }
}

#[inline]
pub fn read_input(filename: &str) -> Vec<Play> {
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    let mut input: Vec<Play> = Vec::new();
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        let line_bytes = line_str.as_bytes();
        let p: (Move, Move, Status) = match (line_bytes[0] as char, line_bytes[2] as char) {
            ('A', 'X') => (Move::Rock, Move::Rock, Status::Lose),
            ('A', 'Y') => (Move::Rock, Move::Paper, Status::Draw),
            ('A', 'Z') => (Move::Rock, Move::Scissors, Status::Win),
            ('B', 'X') => (Move::Paper, Move::Rock, Status::Lose),
            ('B', 'Y') => (Move::Paper, Move::Paper, Status::Draw),
            ('B', 'Z') => (Move::Paper, Move::Scissors, Status::Win),
            ('C', 'X') => (Move::Scissors, Move::Rock, Status::Lose),
            ('C', 'Y') => (Move::Scissors, Move::Paper, Status::Draw),
            ('C', 'Z') => (Move::Scissors, Move::Scissors, Status::Win),
            _ => panic!("Invalid move"),
        };
        input.push(Play { player1: p.0, player2: p.1, status: p.2 });
    }
    input
}

#[inline]
pub fn solve_part1(program: &[Play]) -> u32 {
    program.iter().map(Play::get_score).collect::<Vec<u32>>().iter().sum()
}

#[inline]
pub fn solve_part2(program: &[Play]) -> u32 {
    program.iter().map(Play::get_real_score).collect::<Vec<u32>>().iter().sum()
}
