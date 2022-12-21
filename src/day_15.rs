use crate::*;
use std::{
    collections::HashSet,
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl FromStr for Position {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").ok_or_else(|| format!("invalid position: {s}"))?;
        Ok(Position(
            x.trim_start_matches("x=").parse()?,
            y.trim_start_matches("y=").parse()?,
        ))
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Position {
    const fn manhattan(&self, other: &Self) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

struct Sensor {
    pos: Position,
    beacon: Position,
    beacon_dist: u64,
}

impl FromStr for Sensor {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_coord, beacon_coord) = s
            .trim_start_matches("Sensor at ")
            .split_once(": closest beacon is at ")
            .ok_or_else(|| format!("invalid sensor input: {s}"))?;
        let pos = sensor_coord.parse()?;
        let closest_beacon = beacon_coord.parse()?;
        Ok(Sensor {
            pos,
            beacon: closest_beacon,
            beacon_dist: pos.manhattan(&closest_beacon),
        })
    }
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut sensors = Vec::new();
    for line in reader.lines() {
        let sensor_reading = line?;
        sensors.push(sensor_reading.parse::<Sensor>()?);
    }

    const ROW: i64 = 2000000;
    let mut row_known_beacons = HashSet::new();
    let mut beacon_not_possible_positions = HashSet::new();
    for sensor in sensors.iter() {
        if sensor.beacon.1 == ROW {
            row_known_beacons.insert(sensor.beacon);
        }
        let vertical_distance = sensor.pos.1.abs_diff(ROW);
        if vertical_distance > sensor.beacon_dist {
            continue;
        }
        let horizontal_distance = (sensor.beacon_dist - vertical_distance) as i64;
        for i in 0..=horizontal_distance {
            beacon_not_possible_positions.insert(Position(sensor.pos.0 + i, ROW));
            beacon_not_possible_positions.insert(Position(sensor.pos.0 - i, ROW));
        }
    }
    beacon_not_possible_positions.drain_filter(|pos| row_known_beacons.contains(pos));

    let mut distress_pos = Position(-1, -1);
    const DX: [i64; 4] = [-1, 1, 1, -1];
    const DY: [i64; 4] = [1, 1, -1, -1];
    for sensor in sensors.iter() {
        // Go around the perimeter one point at a time, borrowed from
        // https://www.reddit.com/r/adventofcode/comments/zmi9n4/comment/j0d9nnv/?utm_source=share&utm_medium=web2x&context=3
        let mut perimeter_pos = sensor.pos;
        perimeter_pos.1 -= sensor.beacon_dist as i64 + 1;
        for (dx, dy) in DX.zip(DY) {
            'outer: for _ in 0..=sensor.beacon_dist {
                if perimeter_pos.0 < 0
                    || perimeter_pos.0 > (ROW * 2)
                    || perimeter_pos.1 < 0
                    || perimeter_pos.1 > (ROW * 2)
                {
                    perimeter_pos.0 += dx;
                    perimeter_pos.1 += dy;
                    continue 'outer;
                }
                for sensor in sensors.iter() {
                    if sensor.pos.manhattan(&perimeter_pos) <= sensor.beacon_dist {
                        perimeter_pos.0 += dx;
                        perimeter_pos.1 += dy;
                        continue 'outer;
                    }
                }
                distress_pos = perimeter_pos;
                break;
            }
        }
    }
    let tuning_frequency = (distress_pos.0 * 4000000) + distress_pos.1;

    Ok((
        Box::new(beacon_not_possible_positions.len()),
        Box::new(tuning_frequency),
    ))
}
