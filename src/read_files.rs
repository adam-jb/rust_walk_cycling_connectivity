use fs_err::File;
use serde::de::DeserializeOwned;
use smallvec::SmallVec;
use std::io::BufReader;
use std::time::Instant;

use crate::shared::{EdgeWalk};

pub fn read_files_serial(mode: &String) -> (Vec<i32>, [i8; 32], Vec<Vec<[i32;2]>>, Vec<SmallVec<[EdgeWalk; 4]>>) {
    let now = Instant::now();

    let travel_time_relationships: Vec<i32> =
        deserialize_bincoded_file(&format!("{}_travel_time_relationships_7", mode));

    let subpurpose_purpose_lookup: [i8; 32] =
        deserialize_bincoded_file(&format!("{}_subpurpose_purpose_lookup", mode));
    
    let sparse_node_values: Vec<Vec<[i32;2]>> = deserialize_bincoded_file(&format!("sparse_node_values_6am_{}", &mode));    
    
    let graph_walk: Vec<SmallVec<[EdgeWalk; 4]>> = deserialize_bincoded_file(&format!("p1_main_nodes_list_{}", &mode));

    println!("Serial loading took {:?}", now.elapsed());
    (
        travel_time_relationships,
        subpurpose_purpose_lookup,
        sparse_node_values,
        graph_walk,
    )
}

pub fn deserialize_bincoded_file<T: DeserializeOwned>(filename: &String) -> T {
    let path = format!("serialised_data/{}.bin", filename);
    let file = BufReader::new(File::open(path).unwrap());
    bincode::deserialize_from(file).unwrap()
}

