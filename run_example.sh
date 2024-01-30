#!/bin/bash


./reset.sh 
./generate_RDF.sh n50-p10-t300 50 10 300
sudo ./start_remote_server_simulation.sh
./execute_benchmark.sh all 5
sudo ./stop_remote_server_simulation.sh