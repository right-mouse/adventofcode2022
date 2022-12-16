#![feature(get_mut_unchecked)]

mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

use clap::{CommandFactory, Parser};
use common::*;
use std::{collections::BTreeSet, error::Error, fs::File, io::Write, iter, path::Path};

static SOLVERS: &[SolverFn] = &[
    day_01::solve,
    day_02::solve,
    day_03::solve,
    day_04::solve,
    day_05::solve,
    day_06::solve,
    day_07::solve,
    day_08::solve,
    day_09::solve,
    day_10::solve,
    day_11::solve,
    day_12::solve,
    day_13::solve,
];

#[inline]
fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn error(err: Box<dyn Error>) -> ! {
    _ = Args::command()
        .error(
            clap::error::ErrorKind::Format,
            capitalize_first_letter(&err.to_string()),
        )
        .print();
    _ = std::io::stdout().lock().flush();
    _ = std::io::stderr().lock().flush();
    std::process::exit(1)
}

#[derive(Clone)]
struct Days(Vec<usize>);

fn parse_days(s: &str) -> Result<Days, String> {
    let mut days = BTreeSet::new();
    for part in s.split(',') {
        if let Some(range) = part.split_once('-') {
            let start = clap_num::number_range(range.0, 1, SOLVERS.len())?;
            let end = clap_num::number_range(range.1, 1, SOLVERS.len())?;
            for d in start..=end {
                days.insert(d);
            }
        } else {
            days.insert(clap_num::number_range(part, 1, SOLVERS.len())?);
        }
    }
    Ok(Days(days.into_iter().collect()))
}

fn inputs_from_path(path: &str, days: &[usize]) -> Result<Vec<File>, Box<dyn Error>> {
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
    /// The day(s) (comma separated list including ranges with '-')
    #[clap(long = "day", short = 'd')]
    #[clap(value_parser = parse_days)]
    days: Option<Days>,

    /// The input directory (or file for a single day)
    #[clap(long = "input", short = 'i')]
    #[clap(default_value = "inputs")]
    input: String,
}

fn main() {
    let args = Args::parse();
    let days = args.days.unwrap_or(Days((1..=SOLVERS.len()).collect())).0;
    let inputs = inputs_from_path(&args.input, &days).unwrap_or_else(|err| error(err));
    for (day, file) in days.into_iter().zip(inputs.into_iter()) {
        let results = SOLVERS[day - 1](file).unwrap_or_else(|err| error(err));
        println!("Day {day}");
        println!("  Part 1: {}", results.0.to_string().replace('\n', "\n          "));
        println!("  Part 2: {}", results.1.to_string().replace('\n', "\n          "));
    }
}
