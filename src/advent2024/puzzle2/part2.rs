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

    let increasing = report.windows( 2 )
          .all_but_n( 1,|window| {
            return level_values_close( window ) && window[0] < window[1];
          });

    let decreasing = report.windows( 2 )
          .all_but_n( 1, |window| {
            return level_values_close( window ) && window[1] < window[0];
          });

    return increasing || decreasing;
}

trait IteratorExt<T>: Iterator<Item = T> + Sized {
    /// Checks if no more than n elements of the iterator do not satisfy a predicate.
    /// The built-in all() is equivalent to all_but_n(0).
    fn all_but_n<P>(self, n: u32, predicate: P) -> bool
    where
        P: FnMut(T) -> bool;
}

impl<I, T> IteratorExt<T> for I
where
    I: Iterator<Item = T> + Sized,
{
    fn all_but_n<P>(mut self, n: u32, mut predicate: P) -> bool
    where
        P: FnMut(T) -> bool,
    {
        let mut failures = 0;
        // The implementation can use existing Iterator methods, like next()
        loop {
            match self.next() {
                Some(item) => {
                    if !predicate(item) {
                        failures += 1;
                    }

                    if failures > n {
                        return false;
                    }
                }
                None => return true, // Reached the end, all items passed
            }
        }
    }
}