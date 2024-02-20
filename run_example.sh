#!/bin/bash


# ./reset.sh 
# ./generate_RDF.sh n50-p10-t300 50 10 300
# sudo ./start_remote_server_simulation.sh
# ./execute_benchmark.sh all 5
# sudo ./stop_remote_server_simulation.sh


./reset.sh 
./generate_RDF.sh n1000-p10-t100000 1000 10 100000
./generate_RDF.sh n10000-p10-t1000000 10000 10 1000000
./execute_benchmark.sh local 5