use crate::advent2025::puzzle6::part1::{InputDataError, read_input_data, read_ops_input_data};

pub fn solve() -> Result<u64, InputDataError> {

    let input_string = read_input_data()?;
    let operations = parse_ops_input_data(read_ops_input_data()?.as_str())?;

    let total: u64 = operations
        .iter()
        .map(|op| {
            let values = parse_values(&input_string, op.start_index, op.size)?;

            let result = match op.operation {
                Operation::Add => values.iter().copied().sum(),
                Operation::Multiply => values.iter().copied().product(),
            };

            Ok::<u64,InputDataError>(result)
        })
        .collect::<Result<Vec<u64>, _>>()?
        .iter()
        .sum();

    Ok(total)
}

enum Operation {
    Add,
    Multiply
}

struct OperationAndSize {
    operation: Operation,
    start_index: usize,
    size: usize
}

fn parse_ops_input_data( data_string: &str ) -> Result<Vec<OperationAndSize>,InputDataError> {
    let total_chars = data_string.chars().count();
    let mut index = 1;
    let mut start_index = 0;
    let mut current_column_count = 1;
    let mut current_column_op: Operation = get_operation(data_string.chars().nth(0)).unwrap();

    let mut result: Vec<OperationAndSize> = Vec::new();

    while index < total_chars {
        let operation = get_operation( data_string.chars().nth(index) );
        match operation {
            Some(operation) => {
                // subtract one from current column count to ignore the blank column
                result.push(OperationAndSize { operation: current_column_op, start_index: start_index, size: current_column_count-1 });
                current_column_count = 1;
                current_column_op = operation;
                start_index = index;
            },
            _ => {
                current_column_count = current_column_count+1;
            }
        }
        index = index+1;
    }

    result.push(OperationAndSize { operation: current_column_op, start_index: start_index, size: current_column_count });

    Ok(result)
}

fn parse_values( data_string: &str, start_index: usize, size: usize ) -> Result<Vec<u64>,InputDataError> {
    let data: Vec<&str> = data_string
        .lines()
        .map(|line| &line[start_index..start_index+size])
        .collect::<Vec<&str>>();

    let mut result_values = Vec::new();

    for column_index in 0..size {
        let mut accum = String::new();
        for row_index in 0..data.len() {
            let row_string = data[row_index];
            accum.push_str( &row_string[column_index..=column_index] );
        }
        result_values.push(accum.trim().parse::<u64>()?)
    }

    Ok(result_values)
}

fn get_operation( operation_char: Option<char> ) -> Option<Operation> {
    match operation_char {
        Some('+') => Some(Operation::Add),
        Some('*') => Some(Operation::Multiply),
        _ => None
    }
}