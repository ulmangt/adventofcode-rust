use itertools::Itertools;
use geo::algorithm::contains::Contains;
use geo::{Coord,LineString, Polygon};
use crate::{advent2025::{puzzle9::part1::{InputDataError, calculate_area, parse_input_data, read_input_data}}};

pub fn solve() -> Result<u64,InputDataError> {
    
    let pairs = parse_input_data(read_input_data()?)?;

    let coords = pairs.iter()
         .map(|pair|Coord {x: pair.0 as f64, y:pair.1 as f64})
         .collect();

    let line_string = LineString::new( coords );
    let polygon: Polygon<f64> = Polygon::new(line_string.clone(), vec![]);

    Ok(pairs
         .iter()
         .combinations(2)
         .filter(|combination| polygon.contains(&to_rect(combination[0], combination[1])))
         .map(|combination|calculate_area(combination[0], combination[1]))
         .max()
         .unwrap())

}

fn to_rect( from: &(u32,u32), to: &(u32,u32) ) -> Polygon<f64> {

    let x1 = from.1.min(to.1) as f64;
    let x2 = from.1.max(to.1) as f64;
    let y1 = from.0.min(to.0) as f64;
    let y2 = from.0.max(to.0) as f64;

    Polygon::new(
        LineString::new(vec![
            (x1, y1).into(),
            (x2, y1).into(),
            (x2, y2).into(),
            (x1, y2).into()
        ]),
        vec![],
    )
}