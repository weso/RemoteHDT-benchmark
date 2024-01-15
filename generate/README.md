

# RDF Benchmark Generator

A Rust program for generating RDF (Resource Description Framework) mock database in the N-Triples format. This tool allows users to customize the number of nodes, predicates and number of relations (triplets).

## Usage

To use the program, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/weso/RemoteHDT-benchmark.git
   ```
2. Access this directory:
   ```
   cd generate
   ```
3. If you want to use it using bash:
   ```
   chmod +x generate_output.sh
   ```
   generate_RDF.sh <file_name> <n_nodes> <n_predicates> <n_triples>
   ```
   ./generate_RDF.sh test 100 10 10
   ```
   
