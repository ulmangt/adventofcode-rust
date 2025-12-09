use std::fs;

use crate::{advent2024::puzzle4::part1::Matrix, advent2025::puzzle4::part1::parse_input_data};


pub fn solve() -> Result<usize,InputDataError> {

    let mut manifold: Matrix<char> = parse_input_data( read_input_data()?);

    for row in 0..manifold.rows()-1 {
        step( &mut manifold, row );
        //println!("\n{}", manifold.to_string());
    }

    Ok(manifold
        .iter()
        .filter(|(row,col)| {
            let is_pipe = manifold.get(*row as isize-1, *col as isize).map_or(false, |value| value==&'|');
            let is_splitter = manifold.get_unsigned(*row, *col).map_or(false, |value| value==&'^');
            is_pipe && is_splitter
        })
        .count())
}

fn step( manifold: &mut Matrix<char>, row: usize ) {

    for col in 0..manifold.cols() {
        let value = manifold.get_unsigned(row, col);
        let below_value = manifold.get_unsigned(row+1, col);

        match value {
            Some('S') => {
                manifold.set_unsigned(row+1, col, '|');
            },
            Some('|') => {
                match below_value {
                    Some('^') => {
                        manifold.set_unsigned(row+1, col-1, '|');
                        manifold.set_unsigned(row+1, col+1, '|');
                    },
                    _ => {
                        manifold.set_unsigned(row+1, col, '|');
                    }
                }
            },
            Some('^') => {
                // do nothing
            },
            _ => {
                // do nothing
            }
        }
    }

}

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2025/puzzle7/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
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