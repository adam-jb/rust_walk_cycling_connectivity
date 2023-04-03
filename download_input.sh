#!/bin/bash

set -e

mkdir -p data
mkdir -p serialised_data
 
cd data

for MODE in cycling walk
do
    for x in p1_main_nodes_list_${MODE}.json \
        sparse_node_values_${MODE}.json \
        ${MODE}_subpurpose_purpose_lookup.json \
        ${MODE}_travel_time_relationships_7.json;
    do
        wget https://storage.googleapis.com/hack-bucket-8204707942/$x
    done
done