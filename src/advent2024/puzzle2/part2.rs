use crate::advent2024::puzzle2::part1::{read_input_data,parse_input_data,level_values_close};
use crate::advent2024::puzzle1::part1::InputDataError;


// Solves https://adventofcode.com/2024/day/2#part2
pub fn solve( ) -> Result<usize, InputDataError> {

    let reports = parse_input_data( read_input_data()? )?;

    return Ok( reports.into_iter( )
                       .map( is_report_safe )
                       .filter( |v| *v )
                       .count( ) );
}

fn is_report_safe( report: Vec<u32> ) -> bool {

    let increasing = all_but_1_removed( &report,|window: &[u32]| {
            return level_values_close( window ) && window[0] < window[1];
          });

    let decreasing = all_but_1_removed( &report, |window| {
            return level_values_close( window ) && window[1] < window[0];
          });

    return increasing || decreasing;
}

fn all_but_1_removed( list: &Vec<u32>, predicate: fn(&[u32]) -> bool ) -> bool {

    let apply_predicate = | i1, i2 | {
        return predicate(&[*list.get(i1).unwrap(),*list.get(i2).unwrap()]);
    };

    let mut skip_index = None;
    let mut i = 0;
    while i < list.len()-1 {
        let predicate_value = predicate(&list[i..=i+1]);

        // if the predicate fails and we haven't already failed once,
        // try skipping either the v0 or v1 values
        if skip_index.is_none() && !predicate_value && i < list.len()-2 {
            let p_skip_0 = apply_predicate(i+1, i+2);
            let p_skip_1 = apply_predicate(i,i+2);
            // if both comparisons still fail after skipping, fail immediately
            // as more than one item would need to be removed
            if !p_skip_0 && !p_skip_1 {
                return false;
            }
            // if predicate passed after skipping v0, make sure that doesn't
            // cause comparison between v1 and v-1 to fail
            else if p_skip_0 && i != 0 && !apply_predicate(i-1,i+1) {
                return false;
            }
            // if predicate passed after skipping v1, increment i an extra time
            // to skip over v1, as we are removing it
            else if p_skip_1 {
                i += 1;
                skip_index = Some(i+1);
            }
            else {
                skip_index = Some(i);
            }
        }

        i += 1;
    }

    // handle some special cases at the end of the list
    //
    // if no skips have been made, no need to check the second-last and last values
    // because even if they fail the predicate, we will only have one failure
    //
    // if the second-last index was skipped, no more checks are necessary
    // because the last and third-last were already checked in the last while loop iteration
    if skip_index.is_none() || skip_index.is_some_and( |i| i==list.len()-2 ) {
        return true
    }
    // if we skipped an index besides the second-last, the second-last and last values must pass
    else { 
        return apply_predicate( i-1, i );
    }
}