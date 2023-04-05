# Getting started

1. Run `./download_input.sh` once to download input data

2. Flip the `if false` part of `serialise_files` in `src/main.rs` to `true` so the files are serialised

3. Run with`cargo run --release` to serialise all files. End the process once the API is listening

4. Flip the `if false` part of `serialise_files` in `src/main.rs` to `false` to run without serialising any files

5. Build with `cargo build --release`

6. Run with `./target/release/rust_walk_cycling_connectivity`


# On querying the API

Check it's listening:
```
curl http://0.0.0.0:7328/
```

Run PT algorithm on 3 start nodes: 
```
wget -O- --post-data='{"start_nodes_user_input": [9380647, 9183046, 2420336], "init_travel_times_user_input": [16, 10, 10], "trip_start_seconds": 28800, "graph_walk_additions": [], "graph_pt_additions": [], "new_nodes_count": 0, "graph_walk_updates_keys": [], "graph_walk_updates_additions": [], "year": 2022, "new_build_additions": [], "target_destinations": []}' \
  --header='Content-Type:application/json' \
  'http://0.0.0.0:7328/floodfill_pt/'
```



# Deploying with Docker

To make and run docker image. For networks from 2016 to 2022 the image is 15gb.
```
docker build --progress=plain -t rust_connectivity:latest .
docker run -p 0.0.0.0:7328:7328 rust_connectivity:latest
```

To deploy with Cloud Run do the below, then use Cloud Run UI in GCP to deploy
```
docker build --progress=plain -t rust_connectivity:latest . && \
docker tag rust_connectivity:latest gcr.io/dft-dst-prt-connectivitymetric/adambricknell/connectivity_rust:latest && \
docker push gcr.io/dft-dst-prt-connectivitymetric/adambricknell/connectivity_rust:latest
```

Settings to deploy Cloud Run facing internally
```
Set Ingress control to ‘Internal’ only

Under VPC:
Network: connectivity1
Subnet: serverless
Serverless VPC Access: connector1
Route all traffic through VPC: no

Authentication:
Allow unauthenticated invocations
```



