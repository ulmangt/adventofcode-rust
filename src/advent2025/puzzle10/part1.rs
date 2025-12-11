use std::{fs, num::ParseIntError};

use regex::{Regex};


const REGEX_STRING: &str = r"\[([.#]*)\]((?:\s\(\d+(?:,\d+)*\))*)\s\{(\d+(?:,\d+)*)\}";

pub fn solve() -> Result<u32,InputDataError> {

    let machines: Vec<Machine> = read_input_data()?
        .lines()
        .map(Machine::parse)
        .collect::<Result<Vec<Machine>,InputDataError>>()?;

    Ok(machines.iter().map(Machine::get_fewest_presses).sum())
}

struct Machine {
    indicator_lights_goal: Vec<bool>,
    button_wiring: Vec<Vec<u32>>,
    joltage_requirements: Vec<u32>
}

impl Machine {
    fn parse(line: &str) -> Result<Self,InputDataError> {
        let re = Regex::new(&REGEX_STRING).unwrap();
        println!("{}",line);
        let captures = re.captures(line).ok_or(InputDataError::UnexpectedLineFormat)?;
        let indicator_lights = parse_indicator_lights( captures.get(1).ok_or(InputDataError::UnexpectedLineFormat)?.as_str() );
        println!("{:?}",indicator_lights);
        let button_wiring = parse_button_wiring( captures.get(2).ok_or(InputDataError::UnexpectedLineFormat)?.as_str() )?;
        println!("{:?}",button_wiring);
        let joltage_requirements = parse_joltage_requirements( captures.get(3).ok_or(InputDataError::UnexpectedLineFormat)?.as_str() )?;
        println!("{:?}",joltage_requirements);

        Ok( Machine { indicator_lights_goal: indicator_lights, button_wiring, joltage_requirements } )
    }

    fn get_fewest_presses_helper(&self,indicator_lights:Vec<bool>,presses:u32) -> u32 {
        let mut updated_indicator_lights = Vec::new();

        for buttons in self.button_wiring.iter() {
            let indicator_lights = press_buttons( buttons, &indicator_lights );
            if indicator_lights.eq(&self.indicator_lights_goal) {
                return presses+1;
            }
            updated_indicator_lights.push(indicator_lights);
        }

        updated_indicator_lights
            .iter()
            .map(|indicator_lights|self.get_fewest_presses_helper(indicator_lights.clone(),presses+1))
            .min()
            .unwrap_or(0)
}

    fn get_fewest_presses(&self) -> u32 {
        self.get_fewest_presses_helper( vec![false;self.indicator_lights_goal.len()], 0 )
    }
}

fn press_buttons( buttons: &Vec<u32>,  indicator_lights: &Vec<bool> ) -> Vec<bool> {
    let mut indicator_lights = indicator_lights.clone();
    for button in buttons.iter() {
        indicator_lights.get_mut(*button as usize).map(|value|*value = !*value);
    }
    indicator_lights
}

fn parse_joltage_requirements( string: &str ) -> Result<Vec<u32>,ParseIntError> {
    string
        .trim()
        .split(",")
         .map(|token|token.parse::<u32>())
        .collect()
}

fn parse_button_wiring( string: &str ) -> Result<Vec<Vec<u32>>,ParseIntError> {
    string
        .trim()
        .split_whitespace()
        .map(|csv| {
            csv[1..csv.len()-1]
                .split(",")
                .map(|token|token.parse::<u32>())
                .collect()
        })
        .collect()
}

fn parse_indicator_lights( string: &str ) -> Vec<bool> {
    string.chars()
          .map(|c|{
            match c {
                '.' => false,
                '#' => true,
                _ => false
            }
          })
          .collect()
}

fn read_input_data( ) -> Result<String,std::io::Error> {
    let asset_path: String = format!("{}/assets/2025/puzzle10/part1/test.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string(asset_path);
}

#[derive(Debug)]
pub enum InputDataError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    UnexpectedLineFormat,
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