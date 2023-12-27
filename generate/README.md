

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
3. Use cargo to run it:
   Usage: cargo run <n_nodes> <n_predicates> <n_triples>
   Usage: cargo run <file_name> <n_nodes> <n_predicates> <n_triples>

   ### Examples
   
   ```
   cargo run 10 5 100
   ```
   Will output the file benchmar.nt

   ```
   cargo run benchmark-custom-name 10 5 100
   ```
   Will output the file benchmar.nt

   The files generated are in the outputs folder.
