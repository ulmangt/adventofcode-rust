use crate::{advent2024::puzzle4::part1::Matrix, advent2025::{puzzle4::part1::parse_input_data, puzzle7::part1::{InputDataError, read_input_data, step}}};

pub fn solve() -> Result<u64,InputDataError> {

    let mut manifold: Matrix<char> = parse_input_data( read_input_data()?);

    for row in 0..manifold.rows()-1 {
        step( &mut manifold, row );
        //println!("\n{}", manifold.to_string());
    }

    let mut path_counter: Matrix<u64> = new_matrix( 1, manifold.rows(), manifold.cols());

    for row in (0..manifold.rows()).rev() {
        for col in 0..manifold.cols() {
            let value = manifold.get_unsigned( row, col );

            match value {
                Some('|') => {
                    path_counter.set_unsigned(row-1, col, *path_counter.get_unsigned(row, col).unwrap());
                },
                Some('^') => {
                    path_counter.set_unsigned(row-1, col, *path_counter.get_unsigned(row, col-1).unwrap() + *path_counter.get_unsigned(row, col+1).unwrap());
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    println!("\n{}", path_counter.to_string());

    Ok(*path_counter.get_unsigned(0,get_first_pipe_column( &manifold, 1).unwrap()).unwrap())
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

fn new_matrix( default_value: u64, rows: usize, cols: usize ) -> Matrix<u64> {
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