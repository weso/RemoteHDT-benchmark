#!/bin/bash


# ./reset.sh 
# ./generate_RDF.sh n50-p10-t300 50 10 300
# sudo ./start_remote_server_simulation.sh
# ./execute_benchmark.sh all 5
# sudo ./stop_remote_server_simulation.sh


./reset.sh 
./generate_RDF.sh moreNodes-n1000-p10-t1000000 1000 10 1000000
./generate_RDF.sh moreNodes-n10000-p10-t1000000 10000 10 1000000
./generate_RDF.sh moreNodes-n100000-p10-t1000000 100000 10 1000000
./generate_RDF.sh moreTriples-n10000-p10-t100000 10000 10 100000
./generate_RDF.sh moreTriples-n10000-p10-t1000000 10000 10 1000000
./generate_RDF.sh moreTriples-n10000-p10-t10000000 10000 10 10000000
./execute_benchmark.sh 5 local