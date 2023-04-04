use actix_web::{get, post, web, App, HttpServer};
use rayon::prelude::*;
use smallvec::SmallVec;
use std::time::Instant;

use crate::shared::{Cost, NodeID, UserInputJSON};

use floodfill::{get_scores_and_od_pairs};
use read_files::read_files_serial;

mod floodfill;
mod priority_queue;
mod read_files;
mod serialise_files;
mod shared;


#[get("/")]
async fn index() -> String {
    format!("App is listening")
}


#[post("/floodfill_endpoint/")]
async fn floodfill_endpoint(input: web::Json<UserInputJSON>) -> String {
    
    // Read in files
    let (travel_time_relationships, subpurpose_purpose_lookup, sparse_node_values, graph_walk) =
        read_files_serial(input.mode);
    
    // Extract costs of turning
    if input.mode == "cycling" {
        let time_costs_turn: [u16; 4] = [0, 15, 15, 5];
    } else {
        let time_costs_turn: [u16; 4] = [0, 0, 0, 0];
    }
    
    let now = Instant::now();
    
    let indices = (0..input.start_nodes_user_input.len()).collect::<Vec<_>>();
    
    let results: Vec<(i32, u32, [i64; 32], Vec<u32>, Vec<u16>)> = indices
        .par_iter()
        .map(|i| {
            get_scores_and_od_pairs(
                &travel_time_relationships,
                &subpurpose_purpose_lookup
                &sparse_node_values,
                &graph_walk,
                &time_costs_turn,
                NodeID(*&input.start_nodes_user_input[*i] as u32),
                *&input.trip_start_seconds,
                Cost(*&input.init_travel_times_user_input[*i] as u16),
                &input.target_destinations_vector,
            )
        })
        .collect(); 
    
    println!("Getting destinations and scores took {:?}", now.elapsed());
    
    serde_json::to_string(&results).unwrap()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
        
    // make this true on initial run; false otherwise
    if true {
        serialise_files::serialise_files();
    }

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50)) // allow POST'd JSON payloads up to 50mb
            .service(index)
            .service(floodfill_endpoint)
    })
    .bind(("0.0.0.0", 7328))?
    .run()
    .await
}
