use std::collections::BinaryHeap;
use crate::priority_queue::PriorityQueueItem;
use crate::shared::{Cost, NodeID, Angle, LinkID, EdgeWalk};
use smallvec::SmallVec;
use std::collections::HashSet;

pub fn get_scores_and_od_pairs(
                travel_time_relationships: &[i32],
                subpurpose_purpose_lookup: &[i8; 32],
                sparse_node_values: &Vec<Vec<[i32;2]>>,
                graph_walk: &Vec<SmallVec<[EdgeWalk; 4]>>,
                time_costs_turn: &[u16; 4],
                start: NodeID,
                init_travel_time: Cost,
                target_destinations_vector: &[u32],
            ) -> (i32, u32, [i64; 32], Vec<u32>, Vec<u16>) {
    
    let time_limit: Cost = Cost(3600);
    
    let mut queue: BinaryHeap<PriorityQueueItem<Cost, NodeID, Angle, LinkID>> = BinaryHeap::new();
    queue.push(PriorityQueueItem {
        cost: init_travel_time,
        value: start,
        angle_arrived_from: Angle(0),
        link_arrived_from: LinkID(99_999_999),
    });
                
    let mut scores: [i64; 32] = [0; 32];
    let mut target_destination_travel_times: Vec<u16> = vec![];
    let mut target_destination_ids: Vec<u32> = vec![];
    let mut iters: i32 = 0;
    let mut links_visited = HashSet::new();
                
    let mut target_destinations_binary_vec = vec![false; graph_walk.len() as usize];
    for id in target_destinations_vector.into_iter() {
        target_destinations_binary_vec[*id as usize] = true;
    }

    // catch where start node is over an hour from centroid
    if init_travel_time >= Cost(3600) {
        return (
            iters,
            start.0,
            scores,
            target_destination_ids,
            target_destination_travel_times,
        );
    }
                
    // declare variables which are used below
    let mut time_turn_previous_node: u16;
    let mut angle_turn_previous_node: u16;
                
    
    while let Some(current) = queue.pop() {
        
        if links_visited.contains(&current.link_arrived_from) {
            continue
        }
        links_visited.insert(current.link_arrived_from);
        iters += 1;
 
        // store OD pair
        if target_destinations_binary_vec[current.value.0 as usize] {
            target_destination_ids.push(current.value.0);
            target_destination_travel_times.push(current.cost.0);
        }
        
        // get scores
        for subpurpose_score_pair in sparse_node_values[current.value.0 as usize].iter() {
            let subpurpose_ix = subpurpose_score_pair[0];
            let vec_start_pos_this_purpose = (subpurpose_purpose_lookup[subpurpose_ix as usize] as i32) * 3601;
            let multiplier = travel_time_relationships[(vec_start_pos_this_purpose + current.cost.0 as i32) as usize];
            scores[subpurpose_ix as usize] += (subpurpose_score_pair[1] as i64) * (multiplier as i64);
        }

        for edge in &graph_walk[(current.value.0 as usize)] {
            
            if edge.angle_leaving_node_from < current.angle_arrived_from {
                angle_turn_previous_node = edge.angle_leaving_node_from.0 + 360 - current.angle_arrived_from.0;
            } else {
                angle_turn_previous_node = edge.angle_leaving_node_from.0 -  current.angle_arrived_from.0;
            }
            
            // right turn
            if 45 <= angle_turn_previous_node && angle_turn_previous_node < 135 {
                time_turn_previous_node = time_costs_turn[1];
            // u turn
            } else if 135 <= angle_turn_previous_node && angle_turn_previous_node < 225 {
                time_turn_previous_node = time_costs_turn[2];
            // left turn
            } else if 225 <= angle_turn_previous_node && angle_turn_previous_node < 315 {
               time_turn_previous_node = time_costs_turn[3];
            // no turn
            } else {
                time_turn_previous_node = time_costs_turn[0];
            }
            
            let new_cost = Cost(current.cost.0 + edge.cost.0 + time_turn_previous_node);
            
            if new_cost < time_limit {
                queue.push(PriorityQueueItem {
                    cost: new_cost,
                    value: edge.to,
                    angle_arrived_from: edge.angle_arrived_from,
                    link_arrived_from: edge.link_arrived_from,
                });
            }
        }

    }
                
    return (
        iters,
        start.0,
        scores,
        target_destination_ids,
        target_destination_travel_times,
    );
                
                
}

