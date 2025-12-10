use std::{cmp::Ordering, collections::{BTreeSet, HashMap}, fs, num::ParseIntError};

use itertools::Itertools;


pub fn solve( ) -> Result<u32,InputDataError> {
    let junction_boxes: Vec<JunctionBox> = parse_input_data(read_input_data()?)?;
    let junction_box_pairs: BTreeSet<JunctionBoxPair> = calculate_distances(junction_boxes);

    let mut next_circuit_index: u32 = 0;
    let mut circuit_map: HashMap<JunctionBox,u32> = HashMap::new();

    for junction_box_pair in junction_box_pairs.iter().take(1000) {

        let circuit1 = circuit_map.get( &junction_box_pair.first ).cloned();
        let circuit2 = circuit_map.get( &junction_box_pair.second ).cloned();

        println!("{:?} {:?} {:?}",junction_box_pair,circuit1,circuit2);

        // if both are part of circuits, assign them all to circuit2
        if circuit1.is_some() && circuit2.is_some() {
            let circuit1 = circuit1.unwrap();
            let circuit2 = circuit2.unwrap();
            circuit_map.iter_mut().for_each(|entry| if *entry.1 == circuit1 { *entry.1 = circuit2});
        }
        // only one is part of a circuit, assign the other to that circuit
        else if let Some(circuit) = circuit1 {
            circuit_map.insert(junction_box_pair.second.clone(), circuit);
        }
        else if let Some(circuit) = circuit2 {
            circuit_map.insert(junction_box_pair.first.clone(), circuit);
        }
        // neither are part of circuits, assign both to a new circuit
        else {
            circuit_map.insert(junction_box_pair.first.clone(), next_circuit_index);
            circuit_map.insert(junction_box_pair.second.clone(), next_circuit_index);
            next_circuit_index = next_circuit_index+1;
        }
    }

    let mut circuit_size_map: HashMap<u32,u32> = HashMap::new(); // map circuit index to number of 
    for entry in circuit_map.iter() {
        *circuit_size_map.entry(*entry.1).or_insert(0) += 1;
    }

    println!("{:?}",circuit_size_map);

    Ok(circuit_size_map.values().k_largest(3).product())
}

#[derive(Debug)]
pub struct JunctionBoxPair {
    pub first: JunctionBox,
    pub second: JunctionBox,
    pub distance: f64
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct JunctionBox {
    pub x: u32,
    pub y: u32,
    pub z: u32
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
    let file_path: String = format!("{}/assets/2025/puzzle8/part1/input.txt", env!("CARGO_MANIFEST_DIR"));
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