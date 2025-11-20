use crate::advent2025::puzzle1::part1::{read_input_data,parse_input_data,InputDataError};

pub fn solve( ) -> Result<usize,InputDataError> {
    let input_data = parse_input_data(read_input_data()? )?;

    let similarity_score = input_data.0.iter()
                .map( |value1| input_data.1.iter().filter(|value2| *value1 == **value2 ).count() * (*value1 as usize) )
                .sum();

    return Ok(similarity_score);
}