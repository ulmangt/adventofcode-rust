use std::fs;
use std::{collections::HashMap};

use petgraph::{Directed, Graph};
use petgraph::graph::NodeIndex;

use crate::advent2025::puzzle11::part1::{count_paths, parse_edges, parse_nodes};

pub fn solve() -> Result<u64,std::io::Error> {
    let data: String = read_input_data()?;

    // required pass-through nodes
    let dac: String = String::from("dac");
    let fft: String = String::from("fft");

    // input node
    let svr: String = String::from("svr");
    // output node
    let out: String = String::from("out");


    let mut graph: Graph<String, (), Directed> = Graph::new();
    let mut node_map: HashMap<String,NodeIndex> = HashMap::new();

    parse_nodes(&data)
        .iter()
        .for_each(|node|{
            let idx: NodeIndex = graph.add_node(node.clone());
            node_map.insert(node.clone(), idx);
        });

    node_map.insert( out.clone(), graph.add_node(out.clone()) );

    parse_edges(&data)
        .iter()
        .for_each(|(src, dst)|{
            let src_idx = node_map[src];
            let dst_idx = node_map[dst];
            graph.add_edge(src_idx, dst_idx, ());
        });


    // number of paths from svr -> out passing through both dac and fft in either order is:
    // paths( svr->dac ) * paths( dac->fft ) * paths( fft->out ) + paths( svr->fft ) * paths( fft->dac ) * paths( dac->out )

    let svr_dac = count_paths(&graph,&node_map,&svr,&dac);
    let dac_fft = count_paths(&graph,&node_map,&dac,&fft);
    let fft_out = count_paths(&graph,&node_map,&fft,&out);
    let svr_fft = count_paths(&graph,&node_map,&svr,&fft);
    let fft_dac = count_paths(&graph,&node_map,&fft,&dac);
    let dac_out = count_paths(&graph,&node_map,&dac,&out);

    println!("{svr_dac} * {dac_fft} * {fft_out} + {svr_fft} * {fft_dac} * {dac_out}");

    // to visualize, paste into: https://dreampuf.github.io/GraphvizOnline/
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    Ok(svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out)
}

pub fn read_input_data() -> Result<String,std::io::Error> {
    let input_file_path = format!("{}/assets/2025/puzzle11/part1/input.txt",env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(input_file_path )
}