use crate::*;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    leads_to: Vec<String>,
}

impl Valve {
    fn time_to_reach(valves: &HashMap<String, Self>, src: &str, dest: &str) -> usize {
        path_find(src, dest, valves, &mut HashSet::new(), usize::MAX)
    }
}

fn path_find(
    start: &str,
    end: &str,
    valves: &HashMap<String, Valve>,
    visited: &mut HashSet<String>,
    mut current_min: usize,
) -> usize {
    let current = valves.get(start).unwrap();
    current
        .leads_to
        .iter()
        .filter(|s| !visited.contains(*s))
        .map(|next| {
            if next == end {
                let dist = visited.len() + 1;
                current_min = current_min.min(dist);
                dist
            } else if visited.len() >= (current_min - 1) {
                usize::MAX
            } else {
                let mut new_visited = visited.clone();
                new_visited.insert(next.to_owned());
                path_find(next, end, valves, &mut new_visited, current_min)
            }
        })
        .min()
        .unwrap_or(usize::MAX)
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut valves = HashMap::new();
    let leads_to_re = Regex::new("; tunnels? leads? to valves? ").unwrap();
    for (i, line) in reader.lines().enumerate() {
        let valve_description = line?;
        let (name, rest) = valve_description
            .trim_start_matches("Valve ")
            .split_once(" has flow rate=")
            .ok_or_else(|| format!("malformed input at line {}", i + 1))?;
        let (flow_rate, leads_to) = rest
            .split_once(&leads_to_re)
            .ok_or_else(|| format!("malformed input at line {}", i + 1))?;
        valves.insert(
            name.to_owned(),
            Valve {
                flow_rate: flow_rate.parse()?,
                leads_to: leads_to.split(", ").map(|s| s.to_owned()).collect(),
            },
        );
    }

    let mut to_visit = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate != 0)
        .map(|(name, _)| name.to_owned())
        .collect::<HashSet<_>>();
    let mut time_remaining = 30;
    let mut total_pressure = 0;
    let mut current = "AA".to_owned();
    if to_visit.contains(&current) {
        time_remaining -= 1;
        total_pressure *= valves[&current].flow_rate * time_remaining;
    }
    while !to_visit.is_empty() {
        let mut next = current.clone();
        let mut pressure_released = 0;
        let mut time_taken = 0;
        for valve in to_visit.iter() {
            let time = Valve::time_to_reach(&valves, &current, valve) + 1;
            if time < time_remaining {
                let pressure = (time_remaining - time) * valves[valve].flow_rate;
                println!("  Reaching valve {valve} and opening it will take {time} minutes and will release {pressure} pressure (flow_rate: {})", valves[valve].flow_rate);
                if pressure > pressure_released {
                    next = valve.to_owned();
                    pressure_released = pressure;
                    time_taken = time;
                }
            }
        }
        if next == current {
            println!("No other valve reachable");
            break;
        }
        to_visit.remove(&next);
        println!("Moving from {current} to {next} and opening the valve in {time_taken} minutes for a total pressure of {pressure_released}");
        current = next;
        total_pressure += pressure_released;
        time_remaining -= time_taken;
    }

    Ok((Box::new(total_pressure), Box::new(2)))
}
