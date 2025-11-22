use std::{fs, num::ParseIntError};
use crate::advent2024::puzzle1::part1::InputDataError;


// Solves https://adventofcode.com/2024/day/2
pub fn solve( ) -> Result<usize, InputDataError> {

    let reports = parse_input_data( read_input_data()? )?;

    return Ok( reports.into_iter( )
                       .map( is_report_safe )
                       .filter( |v| *v )
                       .count( ) );
}

fn is_report_safe( report: Vec<u32> ) -> bool {

    let increasing = report.windows( 2 )
          .any( |window| {
            return level_values_close( window ) && window[0] < window[1];
          });

    let decreasing = report.windows( 2 )
          .any( |window| {
            return level_values_close( window ) && window[1] < window[0];
          });

    return increasing || decreasing;
}

fn level_values_close( window: &[u32] ) -> bool {
    let diff = window[0].abs_diff(window[1]);
    return window.len() == 2 && 1 <= diff && diff <= 3;
}

fn read_input_data( ) -> Result<String,std::io::Error> {
    let asset_path = format!("{}/assets/2024/puzzle2/part1/test.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( asset_path );
}

fn parse_input_data( data:String ) -> Result<Vec<Vec<u32>>, ParseIntError> {

    return data.lines( )
                .map( |line| line.split_whitespace()
                                       .map( &str::parse::<u32> )
                                       .collect::<Result<Vec<u32>,ParseIntError>>() )
                .collect::<Result<Vec<Vec<u32>>,ParseIntError>>();
}