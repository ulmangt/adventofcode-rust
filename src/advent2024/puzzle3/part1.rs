use std::fs;
use regex::Regex;
use crate::advent2024::puzzle1::part1::InputDataError;

const REGEX_STRING: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";

pub fn solve() -> Result<u32, InputDataError> {
    let re = Regex::new(REGEX_STRING).unwrap();
    let data = read_input_data()?;

    // collect results into a Result<Vec<_>, _>; the first Err will short-circuit here
    let products: Vec<u32> = re.captures_iter(&data)
        .map(|cap| parse_mul(&cap).map(|(a,b)| a * b))
        .collect::<Result<Vec<u32>, InputDataError>>()?;

    let sum: u32 = products.into_iter().sum();
    Ok(sum)
}


fn parse_mul(cap: &regex::Captures) -> Result<(u32, u32), InputDataError> {
    let g1 = cap.get(1).ok_or(InputDataError::IncorrectListSize("missing capture 1".into()))?;
    let g2 = cap.get(2).ok_or(InputDataError::IncorrectListSize("missing capture 2".into()))?;
    let v1 = g1.as_str().parse::<u32>()?;
    let v2 = g2.as_str().parse::<u32>()?;
    Ok((v1, v2))
}

fn read_input_data( ) -> Result<String,std::io::Error> {
    let asset_path: String = format!("{}/assets/2024/puzzle3/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string(asset_path);
}