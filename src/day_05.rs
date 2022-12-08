use crate::*;
use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut parse_arrangement = true;
    let mut arrangement_a = Vec::new();
    let mut arrangement_b = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let l = line?;
        if parse_arrangement {
            if l.is_empty() {
                parse_arrangement = false;
                arrangement_b = arrangement_a.clone();
                continue;
            }
            if !l.contains('[') {
                continue;
            }
            l.as_bytes()
                .windows(3)
                .enumerate()
                .filter_map(|(j, b)| if j % 4 == 0 { Some(b[1] as char) } else { None })
                .enumerate()
                .for_each(|(idx, c)| {
                    if c != ' ' {
                        if idx >= arrangement_a.len() {
                            arrangement_a.resize_with(idx + 1, VecDeque::new);
                        }
                        arrangement_a[idx].push_front(c);
                    }
                });
        } else {
            let parts = l
                .split(' ')
                .enumerate()
                .filter_map(|(j, n)| if j % 2 != 0 { Some(n.parse::<usize>()) } else { None })
                .collect::<Result<Vec<_>, _>>()?;
            if parts.len() != 3 {
                return Err(format!("malformed input at line {}", i + 1).into());
            }
            let (num, from, to) = (parts[0], parts[1] - 1, parts[2] - 1);
            let mut block = Vec::with_capacity(num);
            for _ in 0..num {
                let c_a = arrangement_a[from]
                    .pop_back()
                    .ok_or_else(|| format!("unable to remove entry from stack {}", from + 1))?;
                arrangement_a[to].push_back(c_a);
                let c_b = arrangement_b[from]
                    .pop_back()
                    .ok_or_else(|| format!("unable to remove entry from stack {}", from + 1))?;
                block.push(c_b);
            }
            block.into_iter().rev().for_each(|c| arrangement_b[to].push_back(c));
        }
    }
    let top_crates_a = arrangement_a
        .iter_mut()
        .enumerate()
        .map(|(i, stack)| {
            stack
                .pop_back()
                .ok_or_else(|| format!("unable to remove entry from stack {}", i + 1))
        })
        .collect::<Result<String, _>>()?;
    let top_crates_b = arrangement_b
        .iter_mut()
        .enumerate()
        .map(|(i, stack)| {
            stack
                .pop_back()
                .ok_or_else(|| format!("unable to remove entry from stack {}", i + 1))
        })
        .collect::<Result<String, _>>()?;
    Ok((Box::new(top_crates_a), Box::new(top_crates_b)))
}
