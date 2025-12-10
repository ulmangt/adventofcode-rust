use itertools::Itertools;

use crate::{advent2024::puzzle4::part1::Matrix, advent2025::{puzzle4::part1::all_pairs_except, puzzle9::part1::{InputDataError, calculate_area, parse_input_data, read_input_data}}};

pub fn solve() -> Result<u64,InputDataError> {
    
    let pairs = parse_input_data(read_input_data()?)?;

    let mut matrix: Matrix<char> = new_matrix( '.', pairs.iter().map(|v|v.1 as usize).max().unwrap(), pairs.iter().map(|v|v.0 as usize).max().unwrap());

    let first_pair = pairs.first().unwrap();
    let mut previous_pair = first_pair;
    for pair in pairs.iter() {
        fill( &mut matrix, previous_pair, pair );
        previous_pair = pair;
    }
    fill( &mut matrix, previous_pair, first_pair );

    println!("{}",matrix.to_string());

    //flood_fill( &mut matrix, 1, 9 );

    //println!("{}",matrix.to_string());

    Ok(pairs.iter().combinations(2).map(|combination|calculate_area(combination[0], combination[1])).max().unwrap())
}

fn flood_fill( matrix: &mut Matrix<char>, row: usize, col: usize ) {

    let mut to_visit: Vec<(usize,usize)> = vec![(row,col)];

    let adjacent = all_pairs_except( -1..=1, -1..=1, vec![(0,0)]);

    while !to_visit.is_empty() {
        let pair = to_visit.pop().unwrap();
        matrix.set_unsigned(pair.0, pair.1, '#');
        adjacent.iter()
                .map(|direction|(pair.0 as i32+direction.0,pair.1 as i32+direction.1))
                .filter(|pair|matrix.get(pair.0 as isize, pair.1 as isize).is_some_and(|value|value!=&'#'))
                .for_each(|pair|to_visit.push((pair.0 as usize,pair.1 as usize)));
    }
}

fn fill( matrix: &mut Matrix<char>, from: &(u32,u32), to: &(u32,u32)) {
    let x_max = from.1.max(to.1) as usize;
    let x_min = from.1.min(to.1) as usize;
    let y_max = from.0.max(to.0) as usize;
    let y_min = from.0.min(to.0) as usize;

    for row in x_min..=x_max {
        for col in y_min..=y_max {
            matrix.set_unsigned(row-1 as usize, col-1 as usize, '#');
        }
    }
}

fn new_matrix( default_value: char, rows: usize, cols: usize ) -> Matrix<char> {
    let mut data = Vec::new();

    for _ in 0..rows {
        let mut col_data = Vec::new();
        for _ in 0..cols {
            col_data.push(default_value);
        }
        data.push(col_data);
    }

    Matrix::new(data)
}