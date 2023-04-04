use smallvec::SmallVec;
use std::time::Instant;

use fs_err::File;
use std::io::BufWriter;

use crate::shared::{Cost, NodeID, Angle, LinkID, EdgeWalk};



pub fn serialise_files() {
    let now = Instant::now();
    
    serialise_graph_walk_vector("walk");
    serialise_graph_walk_vector("cycling");
    
    serialise_sparse_node_values_2d("walk");
    serialise_sparse_node_values_2d("cycling");
    serialise_list_immutable_array_i8("walk_subpurpose_purpose_lookup");
    serialise_list_immutable_array_i8("cycling_subpurpose_purpose_lookup");
    serialise_list("walk_travel_time_relationships_7");
    serialise_list("cycling_travel_time_relationships_7");
    
    println!("File serialisation/tTook {:?}",now.elapsed());
}


fn serialise_graph_walk_vector(mode: String) -> usize {
    let contents_filename = format!("data/p1_main_nodes_list_{}.json", mode);
    let contents = fs_err::read_to_string(contents_filename).unwrap();

    let input: Vec<Vec<[usize; 5]>> = serde_json::from_str(&contents).unwrap();

    let mut graph_walk_vec = Vec::new();
    for input_edges in input.iter() {
        let mut edges: SmallVec<[EdgeWalk; 4]> = SmallVec::new();
        for array in input_edges {
            edges.push(EdgeWalk {
                cost: Cost(array[0] as u16),
                to: NodeID(array[1] as u32),
                angle_leaving_node_from: Angle(array[2] as u16),
                angle_arrived_from: Angle(array[3] as u16),
                link_arrived_from: LinkID(array[4] as u32),
            });
        }
        graph_walk_vec.push(edges);
    }

    let filename = format!("serialised_data/p1_main_nodes_vector_{}.bin", mode);
    let file = BufWriter::new(File::create(filename).unwrap());
    bincode::serialize_into(file, &graph_walk_vec).unwrap();
    return graph_walk_vec.len();
}


fn serialise_sparse_node_values_2d(mode: String) {
    
    let inpath = format!("data/sparse_node_values_{}.json", mode);
    let contents = fs_err::read_to_string(&inpath).unwrap();
    let output: Vec<Vec<[i32;2]>> = serde_json::from_str(&contents).unwrap();
    println!("Read from {}", inpath);

    let outpath = format!("serialised_data/sparse_node_values_{}_2d.bin", mode);
    let file = BufWriter::new(File::create(&outpath).unwrap());
    bincode::serialize_into(file, &output).unwrap();
    println!("Serialised to {}", outpath);
}

fn serialise_list(filename: &str) {
    let inpath = format!("data/{}.json", filename);
    let contents = fs_err::read_to_string(&inpath).unwrap();
    let output: Vec<i32> = serde_json::from_str(&contents).unwrap();
    println!("Read from {}", inpath);

    let outpath = format!("serialised_data/{}.bin", filename);
    let file = BufWriter::new(File::create(&outpath).unwrap());
    bincode::serialize_into(file, &output).unwrap();
    println!("Serialised to {}", outpath);
}

fn serialise_list_immutable_array_i8(filename: &str) {
    let inpath = format!("data/{}.json", filename);
    let contents = std::fs::read_to_string(&inpath).unwrap();
    let output: [i8; 32] = serde_json::from_str(&contents).unwrap();
    println!("Read from {}", inpath);

    let outpath = format!("serialised_data/{}.bin", filename);
    let file = BufWriter::new(File::create(&outpath).unwrap());
    bincode::serialize_into(file, &output).unwrap();
    println!("Serialised to {}", outpath);
}
