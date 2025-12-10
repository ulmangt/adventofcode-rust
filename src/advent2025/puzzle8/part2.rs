use std::collections::{BTreeSet, HashMap};

use crate::advent2025::puzzle8::part1::{InputDataError, JunctionBox, JunctionBoxPair, calculate_distances, parse_input_data, read_input_data};


pub fn solve( ) -> Result<u32,InputDataError> {
    let junction_boxes: Vec<JunctionBox> = parse_input_data(read_input_data()?)?;
    let junction_box_count = junction_boxes.len() as u32;

    let junction_box_pairs: BTreeSet<JunctionBoxPair> = calculate_distances(junction_boxes);

    let mut next_circuit_index: u32 = 0;
    let mut circuit_map: HashMap<JunctionBox,u32> = HashMap::new();

    for junction_box_pair in junction_box_pairs.iter() {

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

        let circuit_size_map = get_circuit_size_map( &circuit_map );

        if circuit_size_map.len() == 1 && circuit_size_map.iter().nth(0).is_some_and(|value|*value.1==junction_box_count) {
            return Ok(junction_box_pair.first.x * junction_box_pair.second.x);
        }
    }

    Ok(0)
}

fn get_circuit_size_map( circuit_map: &HashMap<JunctionBox,u32> ) -> HashMap<u32,u32> {
    let mut circuit_size_map: HashMap<u32,u32> = HashMap::new(); // map circuit index to number of 
    for entry in circuit_map.iter() {
        *circuit_size_map.entry(*entry.1).or_insert(0) += 1;
    }

    circuit_size_map
}