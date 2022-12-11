use std::fs;

pub struct Grid {
    array: Vec<Vec<u8>>,
    size: usize,
}

#[inline]
pub fn read_input(filename: &str) -> Grid {
    let buffer = fs::read_to_string(filename).unwrap();
    let lines = buffer.lines().collect::<Vec<&str>>();
    let size = lines.first().unwrap().len();
    let mut array: Vec<Vec<u8>> = Vec::new();
    for l in lines {
        let mut v: Vec<u8> = Vec::new();
        for z in l.as_bytes() {
            v.push((*z as char).to_digit(10).unwrap() as u8);
        }
        array.push(v);
    }
    Grid { array, size }
}

fn is_visible(program: &Grid, x: usize, y: usize) -> bool {
    let value = program.array[x][y];
    // Up
    let mut up = true;
    for tx in (0..x).rev() {
        if program.array[tx][y] >= value {
            up = false;
            break;
        }
    }
    // Down
    let mut down = true;
    for bx in x + 1..program.size {
        if program.array[bx][y] >= value {
            down = false;
            break;
        }
    }
    // Left
    let mut left = true;
    for ly in (0..y).rev() {
        let next = program.array[x][ly];
        if next >= value {
            left = false;
            break;
        }
    }
    // Right
    let mut right = true;
    for ry in y + 1..program.size {
        if program.array[x][ry] >= value {
            right = false;
            break;
        }
    }
    up || down || left || right
}

fn scenic_score(program: &Grid, x: usize, y: usize) -> u32 {
    let value = program.array[x][y];
    // Up
    let mut up = 0;
    for tx in (0..x).rev() {
        up += 1;
        if program.array[tx][y] >= value {
            break;
        }
    }
    // Down
    let mut down = 0;
    for bx in x + 1..program.size {
        down += 1;
        if program.array[bx][y] >= value {
            break;
        }
    }
    // Left
    let mut left = 0;
    for ly in (0..y).rev() {
        left += 1;
        if program.array[x][ly] >= value {
            break;
        }
    }
    // Right
    let mut right = 0;
    for ry in y + 1..program.size {
        right += 1;
        if program.array[x][ry] >= value {
            break;
        }
    }
    up * left * down * right
}

#[inline]
pub fn solve_part1(program: &Grid) -> u32 {
    let mut result: u32 = (program.size as u32 * 2) + (program.size as u32 - 2) * 2;
    for x in 1..program.size - 1 {
        for y in 1..program.size - 1 {
            if is_visible(program, x, y) {
                result += 1;
            }
        }
    }
    result
}

#[inline]
pub fn solve_part2(program: &Grid) -> u32 {
    let mut result = 0;
    for x in 1..program.size - 1 {
        for y in 1..program.size - 1 {
            let scenic = scenic_score(program, x, y);
            if scenic > result {
                result = scenic;
            }
        }
    }
    result
}
