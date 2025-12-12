use std::{collections::HashMap, fs};

use petgraph::dot::{Config, Dot};
use petgraph::{Directed, Graph};
use petgraph::graph::NodeIndex;

pub fn solve() -> Result<u32,std::io::Error> {
    let data: String = read_input_data()?;

    let mut graph: Graph<String, (), Directed> = Graph::new();
    let mut node_map: HashMap<String,NodeIndex> = HashMap::new();

    parse_nodes(&data)
        .iter()
        .for_each(|node|{
            let idx: NodeIndex = graph.add_node(node.clone());
            node_map.insert(node.clone(), idx);
        });

    node_map.insert( "out".to_owned(), graph.add_node("out".to_owned() ) );

    parse_edges(&data)
        .iter()
        .for_each(|(src, dst)|{
            let src_idx = node_map[src];
            let dst_idx = node_map[dst];
            graph.add_edge(src_idx, dst_idx, ());
        });


    // to visualize, paste into: https://dreampuf.github.io/GraphvizOnline/
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    Ok(0)
}

fn parse_edges( data: &str ) -> Vec<(String,String)> {
    data.lines()
        .flat_map(|line| {
            let mut line_split = line.split(":");
            let from = line_split.next().unwrap().to_owned();
            line_split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|to|(from.clone(),to.trim().to_owned()))
                .collect::<Vec<(String,String)>>()
        })
        .collect()
}

fn parse_nodes( data: &str ) -> Vec<String> {
    data.lines()
        .map(|line|line.split(":").next().unwrap_or("").trim().to_owned())
        .collect()
}

fn read_input_data() -> Result<String,std::io::Error> {
    let input_file_path = format!("{}/assets/2025/puzzle11/part1/test.txt",env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(input_file_path )
}