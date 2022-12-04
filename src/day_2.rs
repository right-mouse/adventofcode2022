use crate::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

impl From<bool> for GameResult {
    fn from(b: bool) -> Self {
        if b {
            GameResult::Win
        } else {
            GameResult::Loss
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<u8> for RPS {
    fn from(i: u8) -> Self {
        match i {
            1 => RPS::Rock,
            2 => RPS::Paper,
            _ => RPS::Scissors,
        }
    }
}

impl RPS {
    #[inline]
    fn base_score(self) -> u32 {
        self as u32
    }

    fn want_result(self, res: GameResult) -> RPS {
        match res {
            GameResult::Draw => self,
            GameResult::Win => (((self as u8) + 1) % 3).into(),
            GameResult::Loss => ((self as u8) - 1).into(),
        }
    }

    fn play(self, other: RPS) -> GameResult {
        if self == other {
            return GameResult::Draw;
        }
        match self {
            RPS::Rock => other == RPS::Scissors,
            RPS::Paper => other == RPS::Rock,
            RPS::Scissors => other == RPS::Paper,
        }
        .into()
    }
}

#[inline]
fn round_score(p1: RPS, p2: RPS) -> u32 {
    let mut score = p2.base_score();
    match p1.play(p2) {
        GameResult::Loss => score += 6,
        GameResult::Draw => score += 3,
        _ => (),
    }
    score
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut total_score_a: u32 = 0;
    let mut total_score_b: u32 = 0;
    for (i, line) in reader.lines().enumerate() {
        let l = line?;
        let (lhs, rhs) = l
            .split_once(' ')
            .ok_or_else(|| format!("malformed input at line {}", i + 1))?;
        let p1 = match lhs {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            _ => Err(format!("malformed input at line {}", i + 1)),
        }?;
        let (p2_a, p2_b) = match rhs {
            "X" => Ok((RPS::Rock, p1.want_result(GameResult::Loss))),
            "Y" => Ok((RPS::Paper, p1.want_result(GameResult::Draw))),
            "Z" => Ok((RPS::Scissors, p1.want_result(GameResult::Win))),
            _ => Err(format!("malformed input at line {}", i + 1)),
        }?;
        total_score_a += round_score(p1, p2_a);
        total_score_b += round_score(p1, p2_b);
    }
    Ok((Box::new(total_score_a), Box::new(total_score_b)))
}
