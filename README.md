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

Run PT algorithm on 5 start nodes for walking: 
```
wget -O- --post-data='{"start_nodes_user_input": [1, 2, 3, 4, 5], "init_travel_times_user_input": [16, 10, 10, 23, 99], "mode": "walk", "target_destinations": [1,2,3,4,55,6,7,8,9,10]}' \
  --header='Content-Type:application/json' \
  'http://0.0.0.0:7328/floodfill_endpoint/'
```

Same, for cycling:
```
wget -O- --post-data='{"start_nodes_user_input": [1, 2, 3, 4, 5], "init_travel_times_user_input": [16, 10, 10, 23, 99], "mode": "cycling", "target_destinations": [1,2,3,4,55,6,7,8,9,10]}' \
  --header='Content-Type:application/json' \
  'http://0.0.0.0:7328/floodfill_endpoint/'
```


# Deploying with Docker

To make and run docker image
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



