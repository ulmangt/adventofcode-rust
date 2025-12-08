use crate::advent2025::puzzle2::part1::{InputDataError, parse_input_data, read_input_data};

pub fn solve( ) -> Result<u64,InputDataError> {

    let ranges = parse_input_data(read_input_data()? )?;

    Ok( ranges.iter()
          .map(|range| {
            (range.0..=range.1)
                 .filter(|code: &u64| is_invalid(*code))
                 .sum::<u64>()
          })
          .sum() )
}

fn is_invalid( code: u64 ) -> bool {
    let code_string = code.to_string();
    let length = code_string.len();
    (1..=length/2)
        .any(|value|is_invalid_single_pattern_size(&code_string,value as usize))

}

fn is_invalid_single_pattern_size( code_string: &str, pattern_size: usize ) -> bool {
    let length = code_string.len();

    // if the string length is not evenly divisible by the pattern size
    // it cannot be made up of repeated strings of that pattern size
    if length % pattern_size != 0 {
        false
    }
    else {
        let pattern: &str = &code_string[0..pattern_size];
        (0..=length-pattern_size)
                    .step_by(pattern_size)
                    .all(|start| &code_string[start..start + pattern_size] == pattern)
    }
}