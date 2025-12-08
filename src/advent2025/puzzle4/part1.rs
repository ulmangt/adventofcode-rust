use std::{fs};
use itertools::Itertools;

use crate::advent2024::puzzle4::part1::{Matrix};

pub fn solve() -> Result<usize,InputDataError> {

    let matrix = parse_input_data(read_input_data()?);

    Ok(matrix.iter()
        // find matrix cells whose value is '@'
        .filter(|coords| matrix.get_tuple_unsigned(coords).map_or(false, |value| value == &'@'))
        // find matrix cells next to less than four '@'
        .filter(|coords| count_adjacent_rolls( &matrix, coords.0 as isize, coords.1 as isize ) < 4)
        .count())
}

pub fn count_adjacent_rolls( matrix: &Matrix<char>, row: isize, col: isize ) -> usize {
    let coords: Vec<(i32, i32)> = all_pairs_except(-1..=1,-1..=1, vec![(0,0)]);

    coords
        .iter()
        .filter(|coords| *matrix.get(row+coords.0 as isize,col+coords.1 as isize).unwrap_or(&'.') == '@')
        .count()
}

fn all_pairs_except<I>( iter1: I, iter2: I, except: Vec<(i32,i32)>) -> Vec<(i32,i32)>
where
    I: Iterator<Item = i32> + Clone,
{
   iter1.cartesian_product(iter2).filter(|pair| !except.contains(pair)).collect()
}

pub fn parse_input_data( string_data: String ) -> Matrix<char> {
    let lines: Vec<&str> = string_data.lines().collect();

    let mut data: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        data.push(row);
    }

    Matrix::new(data)
}

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2025/puzzle4/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
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