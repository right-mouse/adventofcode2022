use crate::*;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    fn move_once(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Position(self.0, self.1 + 1),
            Direction::Down => Position(self.0, self.1 - 1),
            Direction::Left => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0 + 1, self.1),
        }
    }
}

#[inline]
fn update_position(head: Position, tail: Position) -> Position {
    let delta_x = head.0 - tail.0;
    let delta_y = head.1 - tail.1;
    match (delta_x.abs(), delta_y.abs()) {
        (0, 0) | (1, 0) | (0, 1) | (1, 1) => tail,
        (2, 0) => Position(tail.0 + (delta_x / 2), tail.1),
        (0, 2) => Position(tail.0, tail.1 + (delta_y / 2)),
        (2, 1) => Position(tail.0 + (delta_x / 2), tail.1 + delta_y),
        (1, 2) => Position(tail.0 + delta_x, tail.1 + (delta_y / 2)),
        (2, 2) => Position(tail.0 + (delta_x / 2), tail.1 + (delta_y / 2)),
        _ => tail,
    }
}

#[inline]
fn update_rope(dir: Direction, rope: &mut [Position], rope_set: &mut HashSet<Position>) {
    rope[0] = rope[0].move_once(dir);
    for i in 1..rope.len() {
        rope[i] = update_position(rope[i - 1], rope[i]);
    }
    rope_set.insert(rope[rope.len() - 1]);
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut short_rope = [Position(0, 0); 2];
    let mut long_rope = [Position(0, 0); 10];
    let mut short_rope_set = HashSet::new();
    let mut long_rope_set = HashSet::new();
    short_rope_set.insert(short_rope[short_rope.len() - 1]);
    long_rope_set.insert(long_rope[long_rope.len() - 1]);
    for (i, line) in reader.lines().enumerate() {
        let motion = line?;
        let parts = motion
            .split_once(' ')
            .ok_or_else(|| format!("malformed input at line {}", i + 1))?;
        let dir = match parts.0 {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(format!("malformed input at line {}", i + 1).into()),
        };
        let steps = parts.1.parse::<usize>()?;
        for _ in 0..steps {
            update_rope(dir, &mut short_rope, &mut short_rope_set);
            update_rope(dir, &mut long_rope, &mut long_rope_set);
        }
    }
    Ok((Box::new(short_rope_set.len()), Box::new(long_rope_set.len())))
}
