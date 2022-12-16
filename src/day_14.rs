use itertools::Itertools;

use crate::*;
use std::{
    cmp::{max, min},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
const fn map_col(col: usize) -> usize {
    col - 300
}

#[derive(Clone)]
struct Cave {
    grid: [[bool; 400]; 200], // Columns range from 300-700.
    sand_source: (usize, usize),
    lowest_rock: usize,
    has_floor: bool,
}

impl Default for Cave {
    fn default() -> Self {
        Self {
            grid: [[false; 400]; 200],
            sand_source: (map_col(500), 0),
            lowest_rock: 0,
            has_floor: false,
        }
    }
}

impl Cave {
    fn add_line(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        for (start, end) in line.split(" -> ").tuple_windows() {
            let start_parts = start.split_once(',').ok_or_else(|| format!("invalid coord: {start}"))?;
            let end_parts = end.split_once(',').ok_or_else(|| format!("invalid coord: {end}"))?;
            let p0: (usize, usize) = (map_col(start_parts.0.parse()?), start_parts.1.parse()?);
            let p1: (usize, usize) = (map_col(end_parts.0.parse()?), end_parts.1.parse()?);
            if p0.0 == p1.0 {
                // Same col.
                for i in min(p0.1, p1.1)..=max(p0.1, p1.1) {
                    self.grid[i][p0.0] = true;
                    self.lowest_rock = self.lowest_rock.max(i);
                }
            } else if p0.1 == p1.1 {
                // Same row.
                for j in min(p0.0, p1.0)..=max(p0.0, p1.0) {
                    self.grid[p0.1][j] = true;
                }
                self.lowest_rock = self.lowest_rock.max(p0.1);
            }
        }
        Ok(())
    }

    fn add_floor(&mut self) {
        for j in 0..self.grid[0].len() {
            self.grid[self.lowest_rock + 2][j] = true;
        }
        self.has_floor = true;
    }

    fn add_sand(&mut self) -> bool {
        if self.has_floor && self.grid[self.sand_source.1][self.sand_source.0] {
            return false;
        }
        let mut sand_coord = self.sand_source;
        loop {
            if !self.has_floor && sand_coord.1 > self.lowest_rock {
                return false;
            }
            if !self.grid[sand_coord.1 + 1][sand_coord.0] {
                sand_coord = (sand_coord.0, sand_coord.1 + 1);
                continue;
            }
            if !self.grid[sand_coord.1 + 1][sand_coord.0 - 1] {
                sand_coord = (sand_coord.0 - 1, sand_coord.1 + 1);
                continue;
            }
            if !self.grid[sand_coord.1 + 1][sand_coord.0 + 1] {
                sand_coord = (sand_coord.0 + 1, sand_coord.1 + 1);
                continue;
            }
            break;
        }
        self.grid[sand_coord.1][sand_coord.0] = true;
        true
    }
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut cave: Cave = Default::default();
    for line in reader.lines() {
        cave.add_line(&line?)?;
    }
    let mut cave_with_floor = cave.clone();
    cave_with_floor.add_floor();

    let mut num_sand = 0;
    while cave.add_sand() {
        num_sand += 1;
    }

    let mut num_sand_with_floor = 0;
    while cave_with_floor.add_sand() {
        num_sand_with_floor += 1;
    }

    Ok((Box::new(num_sand), Box::new(num_sand_with_floor)))
}
