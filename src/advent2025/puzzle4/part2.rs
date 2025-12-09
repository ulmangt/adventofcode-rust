use crate::{advent2024::puzzle4::part1::Matrix, advent2025::puzzle4::part1::{InputDataError, count_adjacent_rolls, parse_input_data, read_input_data}};

pub fn solve() -> Result<usize,InputDataError> {

    let mut matrix = parse_input_data(read_input_data()?);

    let inital_rolls = count_rolls( &matrix );

    loop {
        let coords_to_remove: Vec<(usize,usize)> = get_accessible_coords(&matrix).collect();
        if coords_to_remove.len() == 0 { break }
        remove_coords(&mut matrix, coords_to_remove.into_iter());
    }

    let final_rolls = count_rolls( &matrix );

    Ok(inital_rolls-final_rolls)
}

fn count_rolls( matrix: &Matrix<char> ) -> usize {
    matrix.iter()
        .filter(|coords| matrix.get_tuple_unsigned(coords).map_or(false, |value| value == &'@'))
        .count()
}

fn remove_coords( matrix: &mut Matrix<char>, coords_to_remove: impl Iterator<Item =(usize,usize)>) {
    coords_to_remove
        .for_each(|coord| matrix.set_tuple_unsigned(&coord, '.'));
}

fn get_accessible_coords( matrix: &Matrix<char> )-> impl Iterator<Item =(usize,usize)>  + '_ {
    
    matrix.iter()
        // find matrix cells whose value is '@'
        .filter(|coords| matrix.get_tuple_unsigned(coords).map_or(false, |value| value == &'@'))
        // find matrix cells next to less than four '@'
        .filter(move |coords| count_adjacent_rolls( &matrix, coords.0 as isize, coords.1 as isize ) < 4)
}