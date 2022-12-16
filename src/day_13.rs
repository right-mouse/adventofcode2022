use itertools::{EitherOrBoth, Itertools};

use crate::*;
use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => lhs.cmp(rhs),
            (Self::List(lhs), Self::List(rhs)) => {
                for item in lhs.iter().zip_longest(rhs.iter()) {
                    match item {
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                        EitherOrBoth::Right(_) => return Ordering::Less,
                        EitherOrBoth::Both(lhs, rhs) => {
                            let ord = lhs.cmp(rhs);
                            if ord != Ordering::Equal {
                                return ord;
                            }
                        }
                    }
                }
                Ordering::Equal
            }
            (Self::Integer(lhs), Self::List(_)) => Packet::List(vec![Packet::Integer(*lhs)]).cmp(other),
            (Self::List(_), Self::Integer(rhs)) => self.cmp(&Packet::List(vec![Packet::Integer(*rhs)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') && s.ends_with(']') {
            let list_contents = &s[1..(s.len() - 1)];
            let mut list = Vec::new();
            let mut current_start = 0;
            let mut nest = 0;
            for (i, c) in list_contents.chars().enumerate() {
                match c {
                    '[' => nest += 1,
                    ']' => nest -= 1,
                    ',' => {
                        if nest == 0 {
                            list.push(list_contents[current_start..i].parse()?);
                            current_start = i + 1;
                        }
                    }
                    _ => (),
                }
            }
            if !list_contents[current_start..].is_empty() {
                list.push(list_contents[current_start..].parse()?);
            }
            Ok(Self::List(list))
        } else {
            Ok(Self::Integer(s.parse::<u8>().map_err(|err| format!("{err}: \"{s}\""))?))
        }
    }
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut pairs = Vec::new();
    while let Some(line) = lines.next() {
        let l = line?;
        if l.is_empty() {
            continue;
        }
        let l2 = lines.next().ok_or_else(|| format!("expected line after {l}"))??;
        pairs.push((l.parse::<Packet>()?, l2.parse::<Packet>()?));
    }

    let in_order_total: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (lhs, rhs))| if lhs < rhs { Some(i + 1) } else { None })
        .sum();

    let divider_packet_1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_packet_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    let mut all_packets = pairs
        .iter()
        .flat_map(|pair| [pair.0.clone(), pair.1.clone()])
        .chain([divider_packet_1.clone(), divider_packet_2.clone()])
        .collect::<Vec<_>>();
    all_packets.sort();
    let mut decoder_key = 1;
    for (i, packet) in all_packets.into_iter().enumerate() {
        if packet == divider_packet_1 || packet == divider_packet_2 {
            decoder_key *= i + 1;
        }
    }

    Ok((Box::new(in_order_total), Box::new(decoder_key)))
}
