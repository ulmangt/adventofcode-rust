use std::{collections::HashMap, fs, num::ParseIntError};

use regex::Regex;

use crate::advent2024::puzzle4::part1::Matrix;

pub fn solve() -> Result<InputData, InputDataError> {
    let data = parse_input_data(read_input_data()?)?;

    Ok(data)
}

const PRESENT_ID_REGEX: &str = r"^(\d+):";
const REGION_REGEX: &str = r"^(\d+)x(\d+):((?:\s\d+)+)";

#[derive(Debug)]
pub struct InputData {
    pub presents: HashMap<u32, Matrix<bool>>,
    pub regions: Vec<Region>,
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
                println!("{}", index);
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

    Ok(InputData { presents, regions })
}

fn parse_presents(lines: &Vec<&str>, lines_index: usize) -> Matrix<bool> {
    let mut data: Matrix<bool> = new_matrix(false, 3, 3);

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

fn new_matrix(default_value: bool, rows: usize, cols: usize) -> Matrix<bool> {
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
