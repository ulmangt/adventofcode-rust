use std::fs;

pub fn solve( ) -> Result<u64,InputDataError> {
    let banks = parse_input_data(read_input_data()?)?;

    Ok(banks.iter()
         .map(|bank| get_joltage(&bank))
         .sum())
}

pub fn read_input_data() -> Result<String,std::io::Error> {
    let file_path = format!("{}/assets/2025/puzzle3/part1/test.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}

pub fn parse_input_data( string: String ) -> Result<Vec<Vec<u64>>,InputDataError> {
    string.lines()
          .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u64>().map_err(InputDataError::from))
                .collect::<Result<Vec<_>, _>>()
          })
          .collect()
}

fn get_joltage( bank:&[u64] ) -> u64 {

    let length = bank.len();
    (0..length-1)
        .flat_map(|first|
            (first+1..length)
                 .map(move |second| {
                    bank[first]*10+bank[second]
                 }))
        .max()
        .unwrap_or(0)
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