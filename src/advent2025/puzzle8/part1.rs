use std::{cmp::Ordering, collections::BTreeSet, fs, num::ParseIntError};

use itertools::Itertools;


pub fn solve( ) -> Result<u32,InputDataError> {
    let junction_boxes: Vec<JunctionBox> = parse_input_data(read_input_data()?)?;
    let junction_box_pairs: BTreeSet<JunctionBoxPair> = calculate_distances(junction_boxes);

    junction_box_pairs.iter()
        .for_each(|value|println!("{:?} {:?} {:?}",value.distance,value.first,value.second));

    Ok(0)
}

#[derive(Debug)]
pub struct JunctionBoxPair {
    first: JunctionBox,
    second: JunctionBox,
    distance: f64
}

impl JunctionBoxPair {

    fn new( first: JunctionBox, second: JunctionBox ) -> Self {
        let distance = first.dist(&second);
        JunctionBoxPair {
            first,
            second,
            distance
        }
    }
}

impl PartialEq for JunctionBoxPair {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for JunctionBoxPair {}


impl PartialOrd for JunctionBoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for JunctionBoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct JunctionBox {
    x: u32,
    y: u32,
    z: u32
}

impl JunctionBox {

    fn new( x: u32, y:u32, z:u32 ) -> Self {
        JunctionBox { x, y, z }
    }

    fn dist( &self, other: &JunctionBox ) -> f64 {
        let xdiff: f64 = self.x as f64 - other.x as f64;
        let ydiff: f64 = self.y as f64 - other.y as f64;
        let zdiff: f64 = self.z as f64 - other.z as f64;

        f64::sqrt( xdiff * xdiff + ydiff * ydiff + zdiff * zdiff )
    }
}

pub fn calculate_distances( junction_boxes: Vec<JunctionBox> ) -> BTreeSet<JunctionBoxPair> {
    let mut set: BTreeSet<JunctionBoxPair> = BTreeSet::new();

    junction_boxes
        .iter()
        .combinations(2)
        .map(|pair|JunctionBoxPair::new(pair[0].clone(), pair[1].clone()))
        .for_each(|pair|{set.insert(pair);});

    set
}


pub fn parse_input_data(string: String) -> Result<Vec<JunctionBox>, ParseIntError> {
    string
        .lines()
        .map(|line| {
            // Parse each token into a u32
            let nums: Result<Vec<u32>, ParseIntError> =
                line.split(',')
                    .map(|token| token.parse::<u32>())
                    .collect();

            // Map parsed numbers to a JunctionBox
            nums.map(|v| JunctionBox::new( v[0], v[1], v[2] ) )
        })
        .collect() // Collect into Result<Vec<JunctionBox>, ParseIntError>
}

pub fn read_input_data( ) -> Result<String,std::io::Error> {
    let file_path: String = format!("{}/assets/2025/puzzle8/part1/test.txt", env!("CARGO_MANIFEST_DIR"));
    return fs::read_to_string( file_path );
}

#[derive(Debug)]
pub enum InputDataError {
    Io(std::io::Error),
    ParseIntError(std::num::ParseIntError),
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