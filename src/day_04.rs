use crate::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut num_redundant_pairs: u32 = 0;
    let mut num_overlapping_pairs: u32 = 0;
    for (i, line) in reader.lines().enumerate() {
        let section_assignment = line?;
        let parts = section_assignment
            .split(&['-', ','])
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        let (start_1, end_1, start_2, end_2) = (parts.len() == 4)
            .then(|| (parts[0], parts[1], parts[2], parts[3]))
            .ok_or_else(|| format!("malformed input at line {}", i + 1))?;
        if (start_1 <= start_2 && end_1 >= end_2) || (start_2 <= start_1 && end_2 >= end_1) {
            num_redundant_pairs += 1;
        }
        if (start_1 <= start_2 && end_1 >= start_2) || (start_2 <= start_1 && end_2 >= start_1) {
            num_overlapping_pairs += 1;
        }
    }
    Ok((Box::from(num_redundant_pairs), Box::from(num_overlapping_pairs)))
}
