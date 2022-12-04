mod common;
mod day_1;

use clap::Parser;
use common::*;
use std::{error::Error, fs::File, iter, path::Path};

static SOLVERS: &[SolverFn] = &[day_1::solve];

fn day_range(s: &str) -> Result<u8, String> {
    clap_num::number_range(s, 1, 25)
}

fn input_from_path(path: &str, days: &[u8]) -> Result<Vec<File>, Box<dyn Error>> {
    let p = Path::new(path);
    if !p.is_dir() && !p.is_file() {
        return Err(Box::from(format!("cannot access {path:?}: no such file or directory")));
    }
    if p.is_file() {
        if days.len() > 1 {
            return Err(Box::from("cannot use a single input file for multiple days"));
        }
        return iter::once(File::open(p))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.into());
    }
    days.iter()
        .map(|day| -> Result<_, Box<dyn Error>> {
            let file_path = p.join(day.to_string()).join("input");
            if !file_path.is_file() {
                return Err(Box::from(format!("cannot open {file_path:?}: no such file")));
            }
            File::open(file_path).map_err(|e| e.into())
        })
        .collect()
}

#[derive(Parser)]
struct Args {
    /// The day (1 - 25)
    #[clap(long = "day", short = 'd')]
    #[clap(value_parser = day_range)]
    day: Option<u8>,

    /// The input directory (or file for a single day)
    #[clap(long = "input", short = 'i')]
    #[clap(default_value = "inputs")]
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let days = match args.day {
        Some(d) => vec![d],
        None => (1u8..=25u8).collect(),
    };
    let inputs = input_from_path(&args.input, &days)?;
    for (day, file) in days.into_iter().zip(inputs.into_iter()) {
        let results = SOLVERS[day as usize - 1](file)?;
        println!("Day {day}");
        println!("  Part 1: {}", results.0);
        println!("  Part 2: {}", results.1);
    }
    Ok(())
}
