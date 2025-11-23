use std::fs;

pub fn solve( ) -> Result<usize,std::io::Error> {
    let matrix = parse_input_data( read_input_data( )? );

    println!("{:?}", matrix);

    println!("{:?}", matrix.get(1, 2).unwrap_or(&'X'));

    println!("{:?}", all_pairs_except( -1..=1, -1..=1, vec![(0,0)] ) );

    //let v: Vec<(i8,i8)> = all_pairs( -1..=1, -1..=1 ).into_iter().filter(|pair| *pair!=(0,0)).collect();

    return Ok(search_pattern(&matrix,"XMAS",&all_pairs_except( -1..=1, -1..=1, vec![(0,0)] ) ) );
}

#[derive(Debug)]
struct Matrix<D> {
    data: Vec<Vec<D>>
}

impl <D> Matrix<D> {
    fn get( &self, row: usize, col: usize ) -> Option<&D> {
        return self.data.get(row).and_then(|row_vec| row_vec.get(col));
    }

    fn rows( &self ) -> usize {
        return self.data.len( );
    }

    fn cols( &self ) -> usize {
        return self.data.get(0).map_or(0, Vec::len);
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows();
        let cols = self.cols();
        (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c)))
    }
}

fn all_pairs_except<I>( iter1: I, iter2: I, except: Vec<(i8,i8)>) -> Vec<(i8,i8)>
where
    I: Iterator<Item = i8> + Clone,
{
    all_pairs(iter1, iter2).into_iter().filter(|pair| !except.contains(pair)).collect()
}


fn all_pairs<I>( iter1: I, iter2: I) -> Vec<(i8,i8)>
where
    I: Iterator<Item = i8> + Clone,
{
    iter1.flat_map(|v1| {
        iter2.clone().map(move |v2| {
            (v1,v2)
        })
    })
    .collect()
}

fn search_pattern( matrix: &Matrix<char>, pattern: &str, directions: &[(i8,i8)] ) -> usize {

    matrix.iter()
          .flat_map(|start| {
                let len = pattern.len();
                directions.iter().copied().map(move |direction| {
                    get_string(matrix, len, direction, start)
                })
                .filter(|s| s == pattern)
          })
          .count()
}

fn get_string(
    matrix: &Matrix<char>,
    length: usize,
    (d_row, d_col): (i8,i8),
    (start_row,start_col): (usize,usize)
) -> String {
    (0..length)
        .filter_map(|index| {
            matrix.get(
                start_row + index * (d_row as usize),
                start_col + index * (d_col as usize)
            )
            .copied()
    })
    .collect::<String>()
}

fn parse_input_data( string_data: String ) -> Matrix<char> {
    let lines: Vec<&str> = string_data.lines().collect();

    let mut data: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        data.push(row);
    }

    return Matrix {
        data
    }
}

fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2024/puzzle4/part1/test.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}

