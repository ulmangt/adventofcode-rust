use std::fs;


pub fn solve( ) -> Result<u32,std::io::Error> {
    let matrix = parse_input_data( read_input_data( )? );

    println!("{:?}", matrix);

    println!("{:?}", matrix.get(1, 2).unwrap_or(&'X'));

    return Ok(0);
}

#[derive(Debug)]
struct Matrix<D> {
    data: Vec<Vec<D>>
}

impl <D> Matrix<D> {
    fn get( &self, row: usize, col: usize ) -> Option<&D> {
        return self.data.get(row).and_then(|row_vec| row_vec.get(col));
    }
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

