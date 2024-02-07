#!/bin/bash


# ./reset.sh 
# ./generate_RDF.sh n50-p10-t300 50 10 300
# sudo ./start_remote_server_simulation.sh
# ./execute_benchmark.sh all 5
# sudo ./stop_remote_server_simulation.sh


./reset.sh 
./generate_RDF.sh n1000000-p1000-t100000000 1000000 1000 100000000
./execute_benchmark.sh local 5