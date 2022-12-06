use crate::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
fn char_idx(c: char) -> usize {
    let code = c as usize;
    if code <= 90 {
        // For A-Z, map ASCII codes 65-90 => 27-52.
        code - 38
    } else {
        // For A-Z, map ASCII codes 97-122 => 1-26.
        code - 96
    }
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut badge_buf: [[bool; 53]; 3] = [[false; 53]; 3]; // Indices 1-26 for a-z, 27-52 for A-Z.
    let mut badge_priority_sum: usize = 0;
    let mut compartment_buf: [[bool; 53]; 2]; // Indices 1-26 for a-z, 27-52 for A-Z.
    let mut compartment_priority_sum: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        let group_num = i % 3;
        if group_num == 0 {
            badge_buf = [[false; 53]; 3];
        }
        compartment_buf = [[false; 53]; 2];
        let rucksack = line?;
        let len = rucksack.len();
        let (compartment_1, compartment_2) = (&rucksack[0..(len / 2)], &rucksack[(len / 2)..len]);
        for (idx_0, idx_1) in compartment_1
            .chars()
            .zip(compartment_2.chars())
            .map(|(ch_0, ch_1)| (char_idx(ch_0), char_idx(ch_1)))
        {
            (compartment_buf[0][idx_0], compartment_buf[1][idx_1]) = (true, true);
            if compartment_buf[1][idx_0] {
                compartment_priority_sum += idx_0;
                break;
            }
            if compartment_buf[0][idx_1] {
                compartment_priority_sum += idx_1;
                break;
            }
        }
        for idx in rucksack.chars().map(char_idx) {
            badge_buf[group_num][idx] = true;
            if group_num == 2 && badge_buf[0][idx] && badge_buf[1][idx] {
                badge_priority_sum += idx;
                break;
            }
        }
    }
    Ok((Box::new(compartment_priority_sum), Box::new(badge_priority_sum)))
}
