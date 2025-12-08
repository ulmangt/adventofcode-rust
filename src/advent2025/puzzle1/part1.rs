use std::{fs};

// Solves https://adventofcode.com/2025/day/1
pub fn solve( ) -> Result<i32,InputDataError> {
    let commands = parse_input_data( read_input_data()? )?;

    Ok( commands
            .iter()
            .fold( ValueAndZeroCount { value: 50, zero_count: 0 }, | acc, element | {
                add_wrapping( acc, *element, 100 )
            }).zero_count)
}

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2025/puzzle1/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}

fn parse_input_data( string: String ) -> Result<Vec<i32>, InputDataError> {
    return string.lines( )
          .map( parse_line )
          .collect();
}

// Parse a string like "L32" into -32 and "R2" into 2
fn parse_line(string: &str) -> Result<i32, InputDataError> {
    let distance_string = string
        .get(1..)
        .ok_or(InputDataError::IncorrectLineLengthError(
            "Line did not contain direction and distance".into(),
        ))?;

    let distance = distance_string.parse::<i32>()?;

    let direction_string = string
        .chars()
        .nth(0)
        .ok_or(InputDataError::IncorrectLineLengthError( "Line did not contain direction and distance".into() ))?;

    let direction = match direction_string {
        'L' => Ok(-1),
        'R' => Ok(1),
        _ => Err(InputDataError::IncorrectDirectionError("Found {direction} instead of \"L\" or \"R\"".into()))
    }?;

    Ok(distance*direction)
}

fn num_wraps( mut increment: i32, max: i32 ) -> i32 {
    increment = i32::abs(increment);
    if increment < max { 0 } else { increment / max }
}

fn add_wrapping( current: ValueAndZeroCount, increment: i32, max: i32 ) -> ValueAndZeroCount {

    let full_wrap_count = num_wraps( increment, max );

    // every multiple of max increments wraps around
    let increment = increment % max;
    let mut new = current.value + increment;
    
    let mut crossed_zero = 0;
    
    if current.value != 0 && new == 0 {
        crossed_zero = 1;
    }
    else if new >= max {
        new -= max;
        if current.value != 0 { crossed_zero = 1; }
    }
    else if new < 0 {
        new += max;
         if current.value != 0 { crossed_zero = 1; }
    }

    ValueAndZeroCount { value: new, zero_count: current.zero_count + full_wrap_count + crossed_zero }
}

struct ValueAndZeroCount {
    value: i32,
    zero_count: i32
}

#[derive(Debug)]
pub enum InputDataError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    IncorrectLineLengthError(String),
    IncorrectDirectionError(String),
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