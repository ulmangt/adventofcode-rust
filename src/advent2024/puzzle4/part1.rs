use std::fs;

pub fn solve( ) -> Result<usize,std::io::Error> {
    let matrix = parse_input_data( read_input_data( )? );
    return Ok(search_pattern(&matrix,"XMAS",&all_pairs_except( -1..=1, -1..=1, vec![(0,0)] ) ) );
}

#[derive(Debug)]
pub struct Matrix<D> {
    data: Vec<Vec<D>>
}

impl <D: ToString> Matrix<D> {
    pub fn new( data: Vec<Vec<D>> ) -> Self {
        Matrix {
            data
        }
    }

    pub fn get_tuple_unsigned( &self, coords: &(usize,usize) ) -> Option<&D> {
        return self.get(coords.0 as isize,coords.1 as isize);
    }

    pub fn get_tuple( &self, coords: &(isize,isize) ) -> Option<&D> {
        return self.get(coords.0,coords.1);
    }

    pub fn get_unsigned( &self, row: usize, col: usize ) -> Option<&D> {
        return self.get( row as isize, col as isize );
    }

    pub fn get( &self, row: isize, col: isize ) -> Option<&D> {
        if row < 0 || col < 0 { return Option::None }
        return self.data.get(row as usize).and_then(|row_vec| row_vec.get(col as usize));
    }

    pub fn set_tuple_unsigned(&mut self,  coords: &(usize,usize), val: D) {
        self.set( coords.0 as isize, coords.1 as isize, val );
    }

    pub fn set_tuple(&mut self,  coords: &(isize,isize), val: D) {
        self.set( coords.0, coords.1, val );
    }

    pub fn set_unsigned(&mut self, row: usize, col: usize, val: D) {
        self.set(row as isize, col as isize, val);
    }

    pub fn set(&mut self, row: isize, col: isize, val: D) {
        if row < 0 || col < 0 {
            return;
        }

        if let Some(row_vec) = self.data.get_mut(row as usize) {
            if let Some(cell) = row_vec.get_mut(col as usize) {
                *cell = val;
            }
        }
    }
    pub fn rows( &self ) -> usize {
        return self.data.len( );
    }

    pub fn cols( &self ) -> usize {
        return self.data.get(0).map_or(0, Vec::len);
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows();
        let cols = self.cols();
        (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c)))
    }

    pub fn to_string(&self) -> String {
        let mut accum: String = String::new();

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                accum.push_str( &self.get_unsigned(row,col).unwrap().to_string());
            }
            accum.push_str("\n");
        }

        return accum;
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
                start_row as isize + index as isize * d_row as isize,
                start_col as isize + index as isize * d_col as isize
            )
            .copied()
    })
    .collect::<String>()
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

    return Matrix {
        data
    }
}

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2024/puzzle4/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}

