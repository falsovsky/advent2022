use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub struct Play {
    player1: Move,
    player2: Move,
}

impl Play {
    pub fn get_score(&self) -> u32 {
        let mut result: u32 = match self.player2 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        // Tie
        if self.player2 == self.player1 {
            result += 3;
        }

        // Rock beats Scissors
        if self.player2 == Move::Rock && self.player1 == Move::Scissors {
            result += 6;
        }

        // Paper beats Rock
        if self.player2 == Move::Paper && self.player1 == Move::Rock {
            result += 6;
        }

        // Scissors beats Paper
        if self.player2 == Move::Scissors && self.player1 == Move::Paper {
            result += 6;
        }

        result
    }

    pub fn get_real_score(&self) -> u32 {
        let mut result: u32 = 0;

        // Lose
        if self.player2 == Move::Rock {
            result += match self.player1 {
                // Paper beats Rock
                Move::Paper => 1,
                // Rock beats Scissors
                Move::Rock => 3,
                // Scissors beats Paper
                Move::Scissors => 2,
            }
        }

        // Draw
        if self.player2 == Move::Paper {
            result += match self.player1 {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            };
            result += 3;
        }

        // Win
        if self.player2 == Move::Scissors {
            result += match self.player1 {
                // Paper beats Rock
                Move::Rock => 2,
                // Scissors beats Paper
                Move::Paper => 3,
                // Rock beats Scissors
                Move::Scissors => 1,
            };
            result += 6;
        }

        result
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

        let player2: Move = match *split.get(1).unwrap() {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid move"),
        };

        input.push(Play { player1, player2 });
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
