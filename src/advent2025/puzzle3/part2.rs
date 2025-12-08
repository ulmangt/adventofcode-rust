use crate::advent2025::puzzle3::part1::{InputDataError, parse_input_data, read_input_data};


pub fn solve( ) -> Result<u64,InputDataError> {
    let banks = parse_input_data(read_input_data()?)?;

    Ok(banks.iter()
         .map(|bank| get_max_joltage(&bank,12))
         .map(|max|{
            println!( "got max {max}");
            max
         })
         .sum())
}


fn get_max_joltage( bank:&[u64], max_enabled: u32 ) -> u64 {

    let mut enabled_indexes = vec![false;bank.len()];

    let mut enabled_count = 0;
    for digit in (0..=9).rev() {
        for index in 0..bank.len() {
            if !enabled_indexes[index] && bank[index] == digit {
                enabled_indexes[index] = true;
                enabled_count = enabled_count+1;
            }

            if enabled_count == max_enabled {
                return get_joltage(&bank, &enabled_indexes, max_enabled);
            }
        }
    }

    return get_joltage(&bank, &enabled_indexes, max_enabled);
}

fn get_joltage( bank:&[u64], enabled_indexes: &[bool], max_enabled: u32 ) -> u64 {

    let mut joltage: u64 = 0;
    let mut remaining_enabled: u32 = max_enabled;

    for index in 0..bank.len() {
        if enabled_indexes[index] {
            joltage = joltage + bank[index] * (10 as u64).pow(remaining_enabled-1);
            remaining_enabled = remaining_enabled-1;
        }
    }

    joltage
}
