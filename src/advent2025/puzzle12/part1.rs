use std::{
    collections::{HashMap, HashSet},
    fs,
    num::ParseIntError,
};

use regex::Regex;

pub fn solve() -> Result<InputData, InputDataError> {
    let data = parse_input_data(read_input_data()?)?;

    rank_upper_left_fits(&data, &mut Matrix::new(false, 10, 5));

    Ok(data)
}

pub fn debug(data: &InputData) {
    println!("{}", print_matrix(data.presents.get(&5).unwrap(), '.', '#'));
    let mut test_region = Matrix::new(false, 10, 10);
    println!(
        "fits? {}",
        check_piece_fit(&test_region, data.presents.get(&5).unwrap(), 0, 0)
    );
    place_piece(&mut test_region, data.presents.get(&5).unwrap(), 0, 0);
    println!("{}", print_matrix(&test_region, '.', '#'));
    println!(
        "fits? {}",
        check_piece_fit(&test_region, data.presents.get(&5).unwrap(), 1, 2)
    );
    place_piece(&mut test_region, data.presents.get(&5).unwrap(), 1, 2);
    println!("{}", print_matrix(&test_region, '.', '#'));

    println!("Rotations:");

    get_rotations(data.presents.get(&5).unwrap())
        .iter()
        .map(|m| print_matrix(m, '.', '#'))
        .for_each(|v| println!("{}", v));

    data.presents_and_rotations_iter()
        .for_each(|v| println!("{}\n{}", v.0, print_matrix(v.1, '.', '#')));
}

pub fn rank_upper_left_fits(input: &InputData, region: &mut Matrix<bool>) {
    let max_distance: usize = region.rows.max(region.cols);

    for distance in 0..max_distance {
        println!("{}", distance);

        let max_row = distance.min(region.rows - 1);
        let max_col = distance.min(region.cols - 1);

        for row in 0..max_row {
            println!("{} {}", row, max_col);
        }

        for col in 0..max_col {
            println!("{} {}", max_row, col);
        }

        println!("{} {}", max_row, max_col);
    }
}

pub fn place_piece(
    region: &mut Matrix<bool>,
    piece: &Matrix<bool>,
    region_row: usize,
    region_col: usize,
) {
    for piece_row in 0..piece.rows {
        for piece_col in 0..piece.cols {
            if piece.get(piece_row, piece_col) {
                region.set(piece_row + region_row, piece_col + region_col, true);
            }
        }
    }
}

/**
 * check that the provide 3x3 piece fits in the provided region when placed with its upper left corner at region_row,region_col
 */
fn check_piece_fit(
    region: &Matrix<bool>,
    piece: &Matrix<bool>,
    region_row: usize,
    region_col: usize,
) -> bool {
    if region_col + piece.cols > region.cols {
        return false;
    }
    if region_row + piece.rows > region.rows {
        return false;
    }

    for piece_row in 0..piece.rows {
        for piece_col in 0..piece.cols {
            if piece.get(piece_row, piece_col)
                && region.get(piece_row + region_row, piece_col + region_col)
            {
                return false;
            }
        }
    }

    return true;
}

pub fn get_rotations(piece: &Matrix<bool>) -> HashSet<Matrix<bool>> {
    let mut rotated_pieces = HashSet::new();
    let mut piece: Matrix<bool> = piece.clone();
    rotated_pieces.insert(piece.clone());

    for _ in 0..3 {
        piece = rotate_clockwise(&piece);
        rotated_pieces.insert(piece.clone());
    }

    rotated_pieces
}

pub fn rotate_clockwise(piece: &Matrix<bool>) -> Matrix<bool> {
    let mut rotated_piece = Matrix::new(false, piece.rows, piece.cols);

    for row in 0..piece.rows {
        for col in 0..piece.cols {
            let value = piece.get(row, col);
            rotated_piece.set(col, piece.rows - row - 1, value);
        }
    }

    rotated_piece
}

const PRESENT_ID_REGEX: &str = r"^(\d+):";
const REGION_REGEX: &str = r"^(\d+)x(\d+):((?:\s\d+)+)";

#[derive(Debug)]
pub struct InputData {
    pub presents: HashMap<u32, Matrix<bool>>,
    pub presents_and_rotations: HashMap<u32, HashSet<Matrix<bool>>>,
    pub regions: Vec<Region>,
}

impl InputData {
    pub fn presents_and_rotations_iter(&self) -> impl Iterator<Item = (&u32, &Matrix<bool>)> + '_ {
        self.presents_and_rotations
            .iter()
            .flat_map(|(key, pieces)| pieces.iter().map(|piece| (&*key, piece)))
    }
}

#[derive(Debug)]
pub struct Region {
    pub width: u32,
    pub height: u32,
    pub required_presents: Vec<u32>,
}

pub fn parse_input_data(data: String) -> Result<InputData, ParseIntError> {
    let lines: Vec<&str> = data.lines().collect();

    let present_regex: Regex = Regex::new(PRESENT_ID_REGEX).unwrap();
    let region_regex: Regex = Regex::new(REGION_REGEX).unwrap();

    let mut presents: HashMap<u32, Matrix<bool>> = HashMap::new();
    let mut regions: Vec<Region> = Vec::new();

    let mut lines_index = 0;
    while lines_index < lines.len() {
        let line = lines[lines_index];
        lines_index += 1;
        if line.is_empty() {
            continue;
        }

        let mut captures = present_regex.captures_iter(line);
        let capture = captures.next();
        if let Some(capture) = capture {
            if let Some(capture) = capture.get(1) {
                let index = capture.as_str().parse::<u32>()?;
                let data: Matrix<bool> = parse_presents(&lines, lines_index);
                presents.insert(index, data);
                lines_index += 3;
            }
        } else {
            if let Some(captures) = region_regex.captures(line) {
                let width = captures.get(1).unwrap().as_str().trim().parse::<u32>()?;
                let height = captures.get(2).unwrap().as_str().trim().parse::<u32>()?;
                let required_presents = captures
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|v| v.parse::<u32>())
                    .collect::<Result<Vec<u32>, ParseIntError>>()?;

                regions.push(Region {
                    width,
                    height,
                    required_presents,
                });
            }
        }
    }

    let mut presents_and_rotations: HashMap<u32, HashSet<Matrix<bool>>> = HashMap::new();

    presents.iter().for_each(|entry| {
        presents_and_rotations.insert(*entry.0, get_rotations(entry.1));
    });

    Ok(InputData {
        presents,
        presents_and_rotations,
        regions,
    })
}

fn parse_presents(lines: &Vec<&str>, lines_index: usize) -> Matrix<bool> {
    let mut data: Matrix<bool> = Matrix::new(false, 3, 3);

    let line1 = parse_present_line(lines[lines_index]);
    let line2 = parse_present_line(lines[lines_index + 1]);
    let line3 = parse_present_line(lines[lines_index + 2]);

    for col in 0..3 {
        data.set(0, col, line1[col as usize]);
        data.set(1, col, line2[col as usize]);
        data.set(2, col, line3[col as usize]);
    }

    data
}

fn parse_present_line(lines: &str) -> Vec<bool> {
    lines.chars().map(|v| v == '#').collect::<Vec<bool>>()
}

pub fn read_input_data() -> Result<String, std::io::Error> {
    let file_path = format!(
        "{}/assets/2025/puzzle12/part1/test.txt",
        env!("CARGO_MANIFEST_DIR")
    );
    fs::read_to_string(file_path)
}

#[derive(Debug)]
pub enum InputDataError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

impl From<std::io::Error> for InputDataError {
    fn from(err: std::io::Error) -> InputDataError {
        InputDataError::Io(err)
    }
}

impl From<std::num::ParseIntError> for InputDataError {
    fn from(err: std::num::ParseIntError) -> InputDataError {
        InputDataError::ParseIntError(err)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Matrix<D> {
    pub data: Vec<D>,
    pub rows: usize,
    pub cols: usize,
}

impl<D: ToString + Copy> Matrix<D> {
    pub fn new(default: D, rows: usize, cols: usize) -> Self {
        let mut data = Vec::new();
        data.resize(rows * cols, default);
        Matrix { data, rows, cols }
    }

    pub fn set(&mut self, row: usize, col: usize, val: D) {
        self.data[row * self.cols + col] = val;
    }

    pub fn get(&self, row: usize, col: usize) -> D {
        self.data[row * self.cols + col]
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows).flat_map(|r| (0..self.cols).map(move |c| (r, c)))
    }

    pub fn to_string(&self) -> String {
        let mut accum: String = String::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                accum.push_str(&self.get(row, col).to_string());
            }
            accum.push_str("\n");
        }

        return accum;
    }
}

fn print_matrix(matrix: &Matrix<bool>, false_char: char, true_char: char) -> String {
    let char_data = matrix
        .data
        .iter()
        .map(|v| if *v { true_char } else { false_char })
        .collect();

    Matrix {
        data: char_data,
        rows: matrix.rows,
        cols: matrix.cols,
    }
    .to_string()
}
