use std::fs;

use rangemap::RangeInclusiveSet;

use crate::advent2025::puzzle2::part1::{InputDataError, parse_line};


pub fn solve() -> Result<usize,InputDataError> {
    let fresh_map = parse_fresh_range_map( read_fresh_input_data()?)?;
    let available_list = parse_available_data( read_available_input_data()?)?;

    Ok(available_list
        .iter()
        .filter(|value| fresh_map.contains(value))
        .count())
}

pub fn parse_available_data( data_string: String ) -> Result<Vec<u64>,InputDataError> {
    Ok(data_string.lines()
                .map(|line| line.parse::<u64>().map_err(InputDataError::from))
                .collect::<Result<Vec<u64>,InputDataError>>()?)
}


pub fn parse_fresh_range_map( data_string: String ) -> Result<RangeInclusiveSet<u64>,InputDataError> {
    let mut map: RangeInclusiveSet<u64> = RangeInclusiveSet::new();

    data_string
        .lines()
        .map(parse_line)
        .for_each(|range| {
            if let Ok(value) = range {
                map.insert(value.0..=value.1);
            }
        } );

    Ok(map)
}

pub fn read_fresh_input_data( ) -> Result<String,std::io::Error> {
    let fresh_file_path: String = format!("{}/assets/2025/puzzle5/part1/input_fresh.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( fresh_file_path );
}

pub fn read_available_input_data( ) -> Result<String,std::io::Error> {
    let available_file_path: String = format!("{}/assets/2025/puzzle5/part1/input_available.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( available_file_path );
}
