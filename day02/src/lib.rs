use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
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
        // Win
        if (self.player2 == Move::Rock && self.player1 == Move::Scissors) || // Rock beats Scissors
            (self.player2 == Move::Paper && self.player1 == Move::Rock) || // Paper beats Rock
            (self.player2 == Move::Scissors && self.player1 == Move::Paper) // Scissors beats Paper
        {
            self.player2 as u32 + Status::Win as u32
        } else if self.player2 == self.player1 {  
            self.player2 as u32 + Status::Draw as u32 // Draw
        } else { 
            self.player2 as u32 + Status::Lose as u32 // Lose
        }
    }

    pub fn get_real_score(&self) -> u32 {
        if self.status == Status::Lose { // Lose
            match self.player1 {
                Move::Paper => Move::Rock as u32,     // Paper beats Rock
                Move::Rock => Move::Scissors as u32,  // Rock beats Scissors
                Move::Scissors => Move::Paper as u32, // Scissors beats Paper
            }
        } else if self.status == Status::Draw { // Draw
            self.player1 as u32 + self.status as u32
        } else { // Win
            (match self.player1 {
                Move::Rock => Move::Paper as u32,     // Paper beats Rock
                Move::Paper => Move::Scissors as u32, // Scissors beats Paper
                Move::Scissors => Move::Rock as u32,  // Rock beats Scissors
            }) + self.status as u32
        }
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

        let split = line_str.split(' ').collect::<Vec<&str>>();

        let player1: Move = match *split.first().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move"),
        };

        let player2: (Move, Status) = match *split.last().unwrap() {
            "X" => (Move::Rock, Status::Lose),
            "Y" => (Move::Paper, Status:: Draw),
            "Z" => (Move::Scissors, Status:: Win),
            _ => panic!("Invalid move"),
        };

        input.push(Play { player1, player2: player2.0, status: player2.1 });
    }
    input
}

#[inline]
pub fn solve_part1(program: &[Play]) -> u32 {
    let mut result: u32 = 0;
    for game in program {
        result += game.get_score();
    }
    result
}

#[inline]
pub fn solve_part2(program: &[Play]) -> u32 {
    let mut result: u32 = 0;
    for game in program {
        result += game.get_real_score();
    }
    result
}
