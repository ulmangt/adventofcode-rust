use crate::advent2024::puzzle4::part1::{Matrix, parse_input_data, read_input_data};


pub fn solve( ) -> Result<usize,std::io::Error> {
    let matrix = parse_input_data( read_input_data( )? );

    // We will always sample from the matrix in the following patten.
    // Thus, there are four valid extracted strings we are searching for:
    // 1.2
    // .3.
    // 4.5
    let search_patterns = vec!["MSAMS","MMASS","SMASM","SSAMM"];

    return Ok(find_matchs(&matrix,search_patterns));
}
fn find_matchs( matrix: &Matrix<char>, search_patterns: Vec<&str> ) -> usize {

    matrix.iter( )
          .map(|(row, col)| {
            let r = row as isize;
            let c = col as isize;

            // coordinates we want to sample
            let coords = [
                (r - 1, c - 1),
                (r - 1, c + 1),
                (r, c),
                (r + 1, c - 1),
                (r + 1, c + 1),
            ];

            coords
                .iter()
                .filter_map(|&(rr, cc)| {
                    if rr >= 0 && cc >= 0 {
                        matrix.get(rr, cc).copied()
                    } else {
                        None
                    }
                })
                .collect::<String>()

          })
          .filter(|pattern| search_patterns.contains(&pattern.as_str()) )
          .count()
}

