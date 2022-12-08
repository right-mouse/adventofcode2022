use crate::*;
use std::{collections::HashSet, error::Error, fs::File, io::Read};

pub fn solve(mut input: File) -> Result<Solution, Box<dyn Error>> {
    let mut data_stream = Vec::new();
    input.read_to_end(&mut data_stream)?;
    let mut start_of_packet_marker = 0;
    let mut start_of_message_marker = 0;
    for (i, chs) in data_stream.windows(4).enumerate() {
        let mut uniq = HashSet::new();
        if chs.iter().all(|c| uniq.insert(*c)) {
            start_of_packet_marker = i + 4;
            break;
        }
    }
    for (i, chs) in data_stream.windows(14).enumerate() {
        let mut uniq = HashSet::new();
        if chs.iter().all(|c| uniq.insert(*c)) {
            start_of_message_marker = i + 14;
            break;
        }
    }
    Ok((Box::new(start_of_packet_marker), Box::new(start_of_message_marker)))
}
