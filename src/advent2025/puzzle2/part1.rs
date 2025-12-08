use std::fs;

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

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2025/puzzle2/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}

fn parse_input_data( string: String ) -> Result<Vec<(u64,u64)>, InputDataError> {
    return string
          .split(',')
          .map( parse_line )
          .collect();
}

fn parse_line( string: &str ) -> Result<(u64, u64), InputDataError> {
    let values: Vec<u64> = string
        .split('-')
        .map(|token| token.parse::<u64>().map_err(InputDataError::from))
        .collect::<Result<Vec<_>, _>>()?;

    if values.len() != 2 {
        return Err(InputDataError::IncorrectLineFormatError(
            "Expected two integers separated by '-'".into(),
        ));
    }

    Ok((values[0], values[1]))
}

fn is_invalid( code: u64 ) -> bool {
    let code_string = code.to_string();
    let length = code_string.len();

    // if the string length is not even, it cannot contain two repeated numbers
    if length %2 != 0 {
        false
    }
    else {
        let first_half : &str = &code_string[0..length/2];
        let second_half : &str = &code_string[length/2..length];

        first_half.eq(second_half)
    }
}

#[derive(Debug)]
pub enum InputDataError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    IncorrectLineFormatError(String),
}

impl From<std::io::Error> for InputDataError {
    fn from(err: std::io::Error) -> InputDataError {
        InputDataError::Io(err)
    }
}

impl From<std::num::ParseIntError> for InputDataError {
    fn from(err: std::num::ParseIntError) -> InputDataError {
        InputDataError::ParseIntError(err)
    }
}