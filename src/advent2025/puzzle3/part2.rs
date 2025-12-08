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

    let mut stack: Vec<u64> = Vec::new();
    let length = bank.len();
    let mut removes_remaining = length as u32 - max_enabled;

    for index in 0..length {
        while !stack.is_empty() && stack.last().unwrap() < &bank[index] && removes_remaining > 0 {
            stack.pop();
            removes_remaining = removes_remaining-1;
        }
        stack.push(bank[index]); 
    }

    stack.truncate(max_enabled as usize);

    return get_joltage( &stack );
}

fn get_joltage( bank:&[u64] ) -> u64 {

    let mut joltage: u64 = 0;

    for index in 0..bank.len() {
        joltage = joltage + bank[index] * (10 as u64).pow(bank.len() as u32-index as u32-1);
    }

    joltage
}
