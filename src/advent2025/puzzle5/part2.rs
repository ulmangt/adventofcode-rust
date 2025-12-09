use crate::advent2025::{puzzle2::part1::InputDataError, puzzle5::part1::{parse_fresh_range_map, read_fresh_input_data}};


pub fn solve() -> Result<u64,InputDataError> {
    let fresh_map = parse_fresh_range_map( read_fresh_input_data()?)?;

    Ok(fresh_map
            .iter()
            .map(|range| *range.end() - *range.start() + 1)
            .sum())
}