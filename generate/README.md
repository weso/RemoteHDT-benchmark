# RDF Benchmark Generator

A Rust program for generating RDF (Resource Description Framework) benchmarks in the N-Triples format. This tool allows users to customize the number of subjects, predicates, objects, and the probability of generating triples for various combinations.

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
   ```
   cargo run <n_subjects> <n_predicates> <n_objects> <n_triples>
   ```
   Example:
   ```
   cargo run 10 5 10 1000
   ```
   If the n_subjects * n_predicates * n_objects is smaller than the triples you will get repeated triples

   The files generated are in the outputs folder


