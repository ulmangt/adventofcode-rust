use itertools::Itertools;

use crate::advent2025::puzzle3::part1::{InputDataError, parse_input_data, read_input_data};


pub fn solve( ) -> Result<u64,InputDataError> {
    let banks = parse_input_data(read_input_data()?)?;

    Ok(banks.iter()
         .map(|bank| get_max_joltage(&bank,12))
         .sum())
}


fn get_max_joltage( bank:&[u64], num_values: usize ) -> u64 {

    let all_combinations: Vec<Vec<&u64>> = bank.iter().combinations(num_values).collect();

    let max = all_combinations
        .iter()
        .map(|combination|get_joltage(combination))
        .max()
        .unwrap_or(0);

    max
}

fn get_joltage( bank:&[&u64] ) -> u64 {
    (0..bank.len())
        .map(|index| bank[index]*(10 as u64).pow(bank.len() as u32-index as u32 -1))
        .sum()
}
