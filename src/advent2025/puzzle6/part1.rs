use std::fs;

pub fn solve() -> Result<u64,InputDataError> {

    let input_data = parse_input_data( read_input_data()? )?;
    let operations = parse_ops_input_data(read_ops_input_data()?)?;
    let columns = input_data[0].len();
    let rows = input_data.len();

    let mut column_sum = 0;

    for column_index in 0..columns {
        
        let operation = operations[column_index].as_str();
        let mut accum = match operation {
            "*" => 1,
            _ => 0,
        };
        
        for row_index in 0..rows {
            let value = input_data[row_index][column_index];

            accum = match operation {
                "*" => accum * value,
                _ => accum + value,
            };
        }

        column_sum = column_sum + accum;
    }

    Ok(column_sum)
}

fn parse_ops_input_data( data_string: String ) -> Result<Vec<String>,InputDataError> {
    Ok(data_string.split_whitespace().map(|value| value.to_string()).collect::<Vec<String>>())
}

fn parse_input_data( data_string: String ) -> Result<Vec<Vec<u64>>,InputDataError> {
    Ok(data_string
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|token| token.parse::<u64>().map_err(InputDataError::from) )
            .collect::<Result<Vec<u64>,InputDataError>>()
        )
        .collect::<Result<Vec<Vec<u64>>,InputDataError>>()?)
}

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2025/puzzle6/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}


pub fn read_ops_input_data( ) -> Result<String,std::io::Error> {
    let ops_file_path: String = format!("{}/assets/2025/puzzle6/part1/input_ops.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( ops_file_path );
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