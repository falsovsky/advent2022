use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
pub enum Side {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Movement {
    s: Side,
    v: i32,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

pub type Input = Vec<Movement>;

#[inline]
pub fn read_input(filename: &str) -> Input {
    let buffer = fs::read_to_string(filename).unwrap();
    let lines = buffer.lines().collect::<Vec<&str>>();
    let mut movements: Input = Vec::new();
    for line in lines {
        let s = line.split_whitespace().collect::<Vec<&str>>();
        let movement = match s[0] {
            "U" => Movement {
                s: Side::Up,
                v: s[1].parse().unwrap(),
            },
            "D" => Movement {
                s: Side::Down,
                v: s[1].parse().unwrap(),
            },
            "L" => Movement {
                s: Side::Left,
                v: s[1].parse().unwrap(),
            },
            "R" => Movement {
                s: Side::Right,
                v: s[1].parse().unwrap(),
            },
            _ => panic!("Invalid movment"),
        };
        movements.push(movement);
    }
    movements
}

fn move_head(head: &Position, side: &Side) -> Position {
    match side {
        Side::Up => Position {
            x: head.x,
            y: head.y + 1,
        },
        Side::Down => Position {
            x: head.x,
            y: head.y - 1,
        },
        Side::Left => Position {
            x: head.x - 1,
            y: head.y,
        },
        Side::Right => Position {
            x: head.x + 1,
            y: head.y,
        },
    }
}

fn move_tail(head: &Position, tail: &Position) -> Position {
    let diff = Position {
        x: head.x - tail.x,
        y: head.y - tail.y,
    };
    if diff.x.abs() == 2 || diff.y.abs() == 2 {
        return Position {
            x: tail.x + diff.x.signum(),
            y: tail.y + diff.y.signum(),
        };
    }
    *tail
}

#[inline]
pub fn solve_part1(program: &Input) -> u32 {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    for movement in program {
        for _ in 0..movement.v {
            head = move_head(&head, &movement.s);
            tail = move_tail(&head, &tail);
            visited.insert(tail);
        }
    }
    visited.len() as u32
}

#[inline]
pub fn solve_part2(program: &Input) -> u32 {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail: Vec<Position> = vec![Position { x: 0, y: 0 }; 9];
    for movement in program {
        for _ in 0..movement.v {
            head = move_head(&head, &movement.s);
            tail[0] = move_tail(&head, &tail[0]);
            for i in 1..tail.len() {
                tail[i] = move_tail(&tail[i - 1], &tail[i]);
            }
            visited.insert(*tail.last().unwrap());
        }
    }
    visited.len() as u32
}
