use std::fs::{File};
use std::io::{BufRead, BufReader};

// Solves https://adventofcode.com/2024/day/1
pub fn solve( ) -> Result<u32,InputDataError> {
    let input_data = group_input_data(parse_input_data(read_input_data()? )? )?;

    let input_data_sum = input_data.iter()
              .map( |pair| pair.0.abs_diff(pair.1) )
              .sum();

    return Ok( input_data_sum );
}

pub fn read_input_data( ) -> Result<Vec<String>,InputDataError> {
    let asset_path = format!("{}/assets/2025/puzzle1/part1/test.txt", env!("CARGO_MANIFEST_DIR"));

    let file = File::open(asset_path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push( line.expect( "Unable to read line from file" ) );
    }

    return Result::Ok(lines);
}

#[derive(Debug)]
pub enum InputDataError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    IncorrectListSize(String),
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

pub fn group_input_data( (mut line1, mut line2): (Vec<u32>, Vec<u32>) ) -> Result<Vec<(u32,u32)>,InputDataError> {

    line1.sort();
    line2.sort();

    let pairs = line1.into_iter()
         .zip(line2.into_iter())
         .collect();

    return Ok( pairs );
}

pub fn parse_input_data( input_lines: Vec<String> ) -> Result<(Vec<u32>, Vec<u32>),InputDataError> {

    let mut line1: Vec<u32> = Vec::new();
    let mut line2: Vec<u32> = Vec::new();

    for line in input_lines {
        let int_values = line.split_whitespace( )
            .map( |l| l.parse::<u32>() )
            .collect::<Result<Vec<_>, _>>()?;

        if int_values.len( ) != 2 {
            return Err( InputDataError::IncorrectListSize( "Expected two integers per line, received: {line}".into() ) );
        }

        line1.push( int_values[0]);
        line2.push( int_values[1]);
    }

    return Ok((line1, line2));
}