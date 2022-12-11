use crate::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn check_view<'a, I: Iterator<Item = &'a u8>>(height: u8, trees: I) -> (bool, usize) {
    let mut visible = true;
    let mut distance = 0;
    for h in trees {
        distance += 1;
        if *h >= height {
            visible = false;
            break;
        }
    }
    (visible, distance)
}

struct TreeGrid(Vec<Vec<u8>>);

impl TreeGrid {
    fn new() -> Self {
        TreeGrid(Vec::new())
    }

    fn scenic_score(&self, x: usize, y: usize) -> (bool, usize) {
        if x == 0 || y == 0 || x == self.0.len() - 1 || y == self.0[0].len() - 1 {
            return (true, 0);
        }
        let height = self.0[x][y];
        let row = self.0[x].as_slice();
        let col = self.0.iter().map(|r| r[y]).collect::<Vec<_>>();
        let mut is_visible = false;
        let mut scenic_score = 1;
        let slices: [Box<dyn Iterator<Item = &u8>>; 4] = [
            Box::new(row[0..y].iter().rev()),
            Box::new(row[(y + 1)..row.len()].iter()),
            Box::new(col[0..x].iter().rev()),
            Box::new(col[(x + 1)..col.len()].iter()),
        ];
        for slice in slices {
            let (visible, score) = check_view(height, slice);
            if visible {
                is_visible = true;
            }
            scenic_score *= score;
        }
        (is_visible, scenic_score)
    }
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut grid = TreeGrid::new();
    for (i, line) in reader.lines().enumerate() {
        let row = line?;
        grid.0.push(
            row.chars()
                .into_iter()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or_else(|| format!("malformed input at line {}", i + 1))
                })
                .collect::<Result<_, _>>()?,
        );
    }
    let mut num_visible = 0;
    let mut max_scenic_score = 0;
    for x in 0..grid.0.len() {
        for y in 0..grid.0[0].len() {
            let (is_visible, scenic_score) = grid.scenic_score(x, y);
            if is_visible {
                num_visible += 1;
            }
            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }
    Ok((Box::new(num_visible), Box::new(max_scenic_score)))
}
