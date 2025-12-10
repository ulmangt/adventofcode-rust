use std::{fs::{self, read}, num::ParseIntError};

use itertools::Itertools;


pub fn solve() -> Result<u64,InputDataError> {
    
    let pairs = parse_input_data(read_input_data()?)?;

    Ok(pairs.iter().combinations(2).map(|combination|calculate_area(combination[0], combination[1])).max().unwrap())

    /*
    let mut pair1: &(u32,u32) = &pairs[0];
    let mut pair2: &(u32,u32) = &pairs[1];
    let mut current_area: u64 = calculate_area(&pair1, &pair2);

    for pair in pairs.iter() {
        // new area replacing pair1
        let new_area_1 = calculate_area(pair, &pair2);
        // new area replacing pair2
        let new_area_2 = calculate_area(&pair1, pair);

        if new_area_1 > current_area && new_area_1 > new_area_2 {
            pair1 = pair;
            current_area = new_area_1;
        }
        else if new_area_2 > current_area && new_area_2 > new_area_1 {
            pair2 = pair;
            current_area = new_area_2;
        }
    }

    Ok(current_area)
    */

    /*
    let x_max = pairs.iter().map(|p|p.0).max().unwrap();
    let x_min = pairs.iter().map(|p|p.0).min().unwrap();
    let y_max = pairs.iter().map(|p|p.1).max().unwrap();
    let y_min = pairs.iter().map(|p|p.1).min().unwrap();

    println!("{} {} {} {}", x_min, x_max, y_min, y_max );

    Ok(pairs.iter()
         .filter(|pair| pair.0 == x_max || pair.0 == x_min || pair.1 == y_max || pair.1 == y_min )
         .count())
    */
}

pub fn calculate_area( pair1: &(u32,u32), pair2: &(u32,u32) ) -> u64 {
    (pair1.0.abs_diff(pair2.0)+1) as u64 * (pair1.1.abs_diff(pair2.1)+1) as u64
}

pub fn parse_input_data(string: String) -> Result<Vec<(u32,u32)>, ParseIntError> {
    string
        .lines()
        .map(|line| {
            let nums: Result<Vec<u32>, ParseIntError> =
                line.split(',')
                    .map(|token| token.parse::<u32>())
                    .collect();
            nums.map(|v| (v[0],v[1]))
        })
        .collect()
}

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2025/puzzle9/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}

#[derive(Debug)]
pub enum InputDataError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
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