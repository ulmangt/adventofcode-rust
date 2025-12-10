use itertools::Itertools;
use crate::{advent2025::{puzzle9::part1::{InputDataError, calculate_area, parse_input_data, read_input_data}}};

pub fn solve() -> Result<u64,InputDataError> {
    
    let pairs: Vec<(u32, u32)> = parse_input_data(read_input_data()?)?;

    Ok(pairs
         .iter()
         .combinations(2)
         .filter(|combination| !contains_edge_or_midpoint(combination[0], combination[1], &pairs))
         .map(|combination|calculate_area(combination[0], combination[1]))
         .max()
         .unwrap())
}

fn contains_edge_or_midpoint( from: &(u32,u32), to: &(u32,u32), pairs: &Vec<(u32, u32)> ) -> bool {
    let x_max = from.0.max(to.0);
    let x_min = from.0.min(to.0);
    let y_max = from.1.max(to.1);
    let y_min = from.1.min(to.1);

    //println!("{:?} {:?} {} {} {} {} {}",from,to,x_min,x_max,y_min,y_max,calculate_area(from,to));

    let first_pair = pairs.first().unwrap();
    let mut previous_pair: &(u32,u32) = first_pair;
    for pair in pairs.iter() {

        if check_contains( x_min, x_max, y_min, y_max, previous_pair, pair ) {
            return true;
        }

        previous_pair = pair;
    }

    return check_contains( x_min, x_max, y_min, y_max, previous_pair, first_pair );
}

fn check_contains( x_min: u32, x_max: u32, y_min: u32, y_max: u32, previous_pair: &(u32,u32), pair: &(u32,u32) ) -> bool {
    let contains_previous: bool = contains( x_min, x_max, y_min, y_max, previous_pair.0, previous_pair.1 );
    let contains_current: bool = contains( x_min, x_max, y_min, y_max, pair.0, pair.1 );
    let contains_midpoint: bool = contains( x_min, x_max, y_min, y_max, previous_pair.0.min(pair.0) + previous_pair.0.abs_diff(pair.0)/2, previous_pair.1.min(pair.1) + previous_pair.1.abs_diff(pair.1)/2 ); // integer rounding an issue here?

    contains_previous || contains_current || contains_midpoint
}

fn contains( x_min: u32, x_max: u32, y_min: u32, y_max: u32, x: u32, y: u32 ) -> bool {
    x_min < x && x < x_max && y_min < y && y < y_max
}