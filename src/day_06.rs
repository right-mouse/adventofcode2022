use crate::*;
use std::{collections::HashSet, error::Error, fs::File, io::Read};

#[inline]
fn first_marker_occurence(data_stream: &[u8], n: usize) -> usize {
    for (i, chs) in data_stream.windows(n).enumerate() {
        let mut uniq = HashSet::new();
        if chs.iter().all(|c| uniq.insert(*c)) {
            return i + n;
        }
    }
    0
}

pub fn solve(mut input: File) -> Result<Solution, Box<dyn Error>> {
    let mut data_stream = Vec::new();
    input.read_to_end(&mut data_stream)?;
    let start_of_packet_marker = first_marker_occurence(&data_stream, 4);
    let start_of_message_marker = first_marker_occurence(&data_stream, 14);
    Ok((Box::new(start_of_packet_marker), Box::new(start_of_message_marker)))
}
