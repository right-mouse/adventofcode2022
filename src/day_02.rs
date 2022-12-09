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
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<u8> for RockPaperScissors {
    fn from(i: u8) -> Self {
        match i {
            1 => RockPaperScissors::Rock,
            2 => RockPaperScissors::Paper,
            _ => RockPaperScissors::Scissors,
        }
    }
}

impl RockPaperScissors {
    #[inline]
    fn base_score(self) -> u32 {
        self as u32
    }

    fn want_result(self, res: GameResult) -> RockPaperScissors {
        match res {
            GameResult::Draw => self,
            GameResult::Win => (((self as u8) + 1) % 3).into(),
            GameResult::Loss => ((self as u8) - 1).into(),
        }
    }

    fn play(self, other: RockPaperScissors) -> GameResult {
        if self == other {
            return GameResult::Draw;
        }
        match self {
            RockPaperScissors::Rock => other == RockPaperScissors::Scissors,
            RockPaperScissors::Paper => other == RockPaperScissors::Rock,
            RockPaperScissors::Scissors => other == RockPaperScissors::Paper,
        }
        .into()
    }
}

#[inline]
fn round_score(p_1: RockPaperScissors, p_2: RockPaperScissors) -> u32 {
    let mut score = p_2.base_score();
    match p_1.play(p_2) {
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
        let p_1 = match lhs {
            "A" => Ok(RockPaperScissors::Rock),
            "B" => Ok(RockPaperScissors::Paper),
            "C" => Ok(RockPaperScissors::Scissors),
            _ => Err(format!("malformed input at line {}", i + 1)),
        }?;
        let (p_2_a, p_2_b) = match rhs {
            "X" => Ok((RockPaperScissors::Rock, p_1.want_result(GameResult::Loss))),
            "Y" => Ok((RockPaperScissors::Paper, p_1.want_result(GameResult::Draw))),
            "Z" => Ok((RockPaperScissors::Scissors, p_1.want_result(GameResult::Win))),
            _ => Err(format!("malformed input at line {}", i + 1)),
        }?;
        total_score_a += round_score(p_1, p_2_a);
        total_score_b += round_score(p_1, p_2_b);
    }
    Ok((Box::new(total_score_a), Box::new(total_score_b)))
}
