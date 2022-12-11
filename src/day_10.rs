use crate::*;
use std::{
    error::Error,
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
fn update(signal_strength: &mut i32, crt_screen: &mut [[bool; 40]; 6], x: i32, cycle: usize) {
    if cycle > 240 {
        return;
    }
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => *signal_strength += x * (cycle as i32),
        _ => (),
    }
    let row = (cycle - 1) / 40;
    let pos = cycle - (40 * row) - 1;
    crt_screen[row][pos] = (x - (pos as i32)).abs() <= 1;
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut cycle = 1;
    let mut x = 1;
    let mut signal_strength = 0;
    let mut crt_screen = [[false; 40]; 6];
    for (i, line) in reader.lines().enumerate() {
        let command = line?;
        let (instruction, v) = command.split_once(' ').unwrap_or((&command, ""));
        update(&mut signal_strength, &mut crt_screen, x, cycle);
        match instruction {
            "addx" => {
                cycle += 1;
                update(&mut signal_strength, &mut crt_screen, x, cycle);
                x += v.parse::<i32>()?;
                cycle += 1;
            }
            "noop" => cycle += 1,
            _ => return Err(format!("malformed input at line {}", i + 1).into()),
        }
    }
    update(&mut signal_strength, &mut crt_screen, x, cycle);
    let mut crt = String::new();
    for (i, row) in crt_screen.into_iter().enumerate() {
        if i != 0 {
            writeln!(crt)?;
        }
        for p in row {
            write!(crt, "{}", if p { '#' } else { '.' })?;
        }
    }
    Ok((Box::new(signal_strength), Box::new(crt)))
}
