use crate::*;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn a_star(start: &[(usize, usize)], end: (usize, usize), height_map: &Vec<Vec<u8>>) -> usize {
    let rows = height_map.len();
    let cols = height_map[0].len();
    let h = |n: &(usize, usize)| end.0.abs_diff(n.0) + end.1.abs_diff(n.1);

    let mut g_scores = HashMap::new();
    let mut f_scores = HashMap::new();
    let mut open_set = HashSet::new();
    let mut came_from = HashMap::new();

    for &p in start.iter() {
        g_scores.insert(p, 0);
        f_scores.insert(p, h(&p));
        open_set.insert(p);
    }

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by(|&x, &y| {
                let mut f_x = *g_scores.get(x).unwrap_or(&usize::MAX);
                let mut f_y = *g_scores.get(y).unwrap_or(&usize::MAX);
                if f_x != usize::MAX {
                    f_x += h(x);
                }
                if f_y != usize::MAX {
                    f_y += h(y);
                }
                f_x.cmp(&f_y)
            })
            .unwrap();
        open_set.remove(&current);
        if current == end {
            break;
        }

        let current_height = height_map[current.0][current.1];
        let mut neighbours = Vec::new();
        if (current.0 != 0) && (height_map[current.0 - 1][current.1] <= current_height + 1) {
            neighbours.push((current.0 - 1, current.1));
        }
        if (current.0 != rows - 1) && (height_map[current.0 + 1][current.1] <= current_height + 1) {
            neighbours.push((current.0 + 1, current.1));
        }
        if (current.1 != 0) && (height_map[current.0][current.1 - 1] <= current_height + 1) {
            neighbours.push((current.0, current.1 - 1));
        }
        if (current.1 != cols - 1) && (height_map[current.0][current.1 + 1] <= current_height + 1) {
            neighbours.push((current.0, current.1 + 1));
        }

        let tentative_g_score = *g_scores.get(&current).unwrap() + 1;
        for neighbour in neighbours.into_iter() {
            if tentative_g_score < *g_scores.get(&neighbour).unwrap_or(&usize::MAX) {
                came_from.insert(neighbour, current);
                g_scores.insert(neighbour, tentative_g_score);
                f_scores.insert(neighbour, tentative_g_score + h(&neighbour));
                open_set.insert(neighbour);
            }
        }
    }

    *g_scores.get(&end).unwrap()
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut height_map: Vec<Vec<u8>> = Vec::new();
    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut all_starts = Vec::new();
    for line in reader.lines() {
        let row = line?;
        height_map.push(
            row.chars()
                .into_iter()
                .enumerate()
                .map(|(i, c)| {
                    let h = match c {
                        'S' => {
                            start = (height_map.len(), i);
                            0
                        }
                        'E' => {
                            end = (height_map.len(), i);
                            25
                        }
                        _ => (c as u8) - 97, // Map ASCII codes 97-122 => 0-25.
                    };
                    if h == 0 {
                        all_starts.push((height_map.len(), i));
                    }
                    h
                })
                .collect(),
        );
    }

    Ok((
        Box::new(a_star(&[start], end, &height_map)),
        Box::new(a_star(&all_starts, end, &height_map)),
    ))
}
