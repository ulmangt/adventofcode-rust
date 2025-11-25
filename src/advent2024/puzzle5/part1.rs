use std::{collections::{HashMap, HashSet}};
use std::{fs, num::ParseIntError};

pub fn solve( ) -> Result<i32,std::io::Error> {

    // when iterating through rules X|Y, keep map of Y to Set<X> (all Xs prior to Y)
    // when iterating through updates, store previous encountered values in Set
    //      then for each value Z encountered, look up in map and check than previously
    //      encountered values are in the Set assocaited with Z in the map

    let ordering_rules = create_ordering_rules_map( read_ordering_rules_data()? );

   println!("{:?}",ordering_rules);

    Ok(0)
}

fn create_ordering_rules_map( data: String ) -> Result<HashMap<u32,HashSet<u32>>,ParseIntError> {
    data.lines()
        // parse lines like "X|Y" into tuple (X,Y)
        .map(|line| {
            let tokens: Vec<&str> = line.split('|').collect();
            Ok((tokens[0].parse::<u32>()?,tokens[1].parse::<u32>()?))
        })
        // push (X,Y) into Map Y->HashSet(X)
        .try_fold( HashMap::new(), |mut accum: HashMap<u32,HashSet<u32>>,value: Result<(u32, u32), ParseIntError>| {
            value.map(|(first,second)| {
                accum.entry(second)
                     .and_modify(|value: &mut HashSet<u32>| {value.insert(first);} )
                     .or_insert(HashSet::from([first]));
            })?;
            Ok(accum)
        })
}

fn read_ordering_rules_data( ) -> Result<String,std::io::Error> {
    let path_string = format!("{}/assets/2024/puzzle5/part1/input1.txt", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(path_string)
}

fn read_updates_data( ) -> Result<String,std::io::Error> {
    let path_string = format!("{}/assets/2024/puzzle5/part1/input2.txt", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(path_string)
}
