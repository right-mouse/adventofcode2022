use crate::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
fn update_totals(cur_total: u32, top_three_totals: &mut [u32; 3]) {
    for i in 0..3 {
        if cur_total > top_three_totals[i] {
            for j in ((i + 1)..3).rev() {
                top_three_totals[j] = top_three_totals[j - 1];
            }
            top_three_totals[i] = cur_total;
            return;
        }
    }
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut top_three_totals = [0u32; 3];
    let mut cur_total: u32 = 0;
    for line in reader.lines() {
        let calories = line?;
        if calories.is_empty() {
            update_totals(cur_total, &mut top_three_totals);
            cur_total = 0;
        } else {
            cur_total += calories.parse::<u32>()?;
        }
    }
    update_totals(cur_total, &mut top_three_totals);
    Ok((
        Box::new(top_three_totals[0]),
        Box::new(top_three_totals.iter().sum::<u32>()),
    ))
}
