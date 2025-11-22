use std::fs;
use regex::Regex;
use crate::advent2024::puzzle1::part1::InputDataError;

const MUL_REGEX_STRING: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";
const DO_REGEX_STRING: &str = r"do\(\)";
const DONT_REGEX_STRING: &str = r"don't\(\)";


pub fn solve() -> Result<u32, InputDataError> {
    let regex_string = format!("(?:{}|{}|{})", MUL_REGEX_STRING, DO_REGEX_STRING, DONT_REGEX_STRING);

    let re = Regex::new(&regex_string).unwrap();
    let data = read_input_data()?;

    // collect results into a Result<Vec<_>, _>; the first Err will short-circuit here
    let (sum,_) = re.captures_iter(&data)
        .try_fold( (0,true),
               |(acc_sum,acc_on),capture| {
            
            let whole = capture.get(0).map(|m| m.as_str()).unwrap_or(""); 

            
            if whole.starts_with("don't") {
                Ok((acc_sum, false))
            }
            else if whole.starts_with("do") {
                Ok::<(u32,bool),InputDataError>((acc_sum, true))
            }
            else {
                let (v1,v2) = parse_mul(&capture )?;

                if acc_on {
                    Ok((acc_sum + v1*v2,acc_on))
                }
                else {
                    Ok((acc_sum,acc_on))
                }
            }

        })?;

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