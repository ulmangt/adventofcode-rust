use crate::{advent2024::puzzle4::part1::Matrix, advent2025::{puzzle4::part1::parse_input_data, puzzle7::part1::{InputDataError, read_input_data, step}}, main};

pub fn solve() -> Result<usize,InputDataError> {

    let mut manifold: Matrix<char> = parse_input_data( read_input_data()?);

    for row in 0..manifold.rows()-1 {
        step( &mut manifold, row );
        //println!("\n{}", manifold.to_string());
    }

    Ok(count_paths(&manifold, 1, get_first_pipe_column( &manifold, 1).unwrap()))
}

pub fn count_paths( manifold: &Matrix<char>, row: usize, col: usize ) -> usize {
    let value = manifold.get_unsigned( row+1, col );

    match value {
        Some('|') => {
            count_paths( manifold, row+1, col )
        },
        Some('^') => {
            count_paths( manifold, row+1, col-1 ) + count_paths( manifold, row+1, col+1 )
        }
        _ => {
            1
        }
    }
}

fn get_first_pipe_column( manifold: &Matrix<char>, row: usize ) -> Option<usize> {
    for col in 0..manifold.cols() {
        match manifold.get_unsigned(row, col) {
            Some('|') => return Some(col),
            _ => ()
        }
    }

    return None;
}