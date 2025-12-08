use std::{fs};

// Solves https://adventofcode.com/2025/day/1
pub fn solve( ) -> Result<i32,InputDataError> {
    let commands = parse_input_data( read_input_data()? )?;
    Ok( commands
            .iter()
            .fold( (50,0), | acc, element | {
                let new_value = add_wrapping( acc.0, *element, 100 );
                (new_value, if new_value == 0 { acc.1 + 1 } else { acc.1 })
            }).1)
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

fn add_wrapping( current: i32, increment: i32, max: i32 ) -> i32 {
    // every multiple of max increments wraps around
    let increment = increment % max;
    let mut new = current + increment;
    if new >= max { new -= max }
    if new < 0 { new += max }
    new
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