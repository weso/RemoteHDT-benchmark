use rand::Rng;
use remote_hdt::storage::tabular::TabularLayout;
use remote_hdt::storage::ChunkingStrategy;
use remote_hdt::storage::LocalStorage;
use rnglib::{Language, RNG};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::vec::Vec;

//default destination => outputs/benchmark.nt
const FILE_NAME: &str = "benchmark";
//const FOLDER: &str = "outputs";
const FOLDER: &str = "../zarr-files";
//default destination => outputs/root.zarr
const ZARR_NAME: &str = "root";

fn main() {
    let args: Vec<String> = env::args().collect();

    //Different inputs
    match args.len() {
        4 => {
            // Usage: cargo run <n_nodes> <n_predicates> <n_triples>
            let file_path: String = format!("{}/{}.nt", FOLDER, FILE_NAME);
            let zarr_path: String = format!("{}/{}.zarr", FOLDER, ZARR_NAME);
            let n_nodes: i128 = args[1].parse::<i128>().unwrap();
            let n_predicates: i128 = args[2].parse::<i128>().unwrap();
            let n_triples: i128 = args[3].parse::<i128>().unwrap();

            create_benchmark_database(file_path, zarr_path, n_nodes, n_predicates, n_triples);
        }
        5 => {
            //Usage: cargo run <benchmark_name> <n_nodes> <n_predicates> <n_triples>
            let file_path: String = format!("{}/{}.nt", FOLDER, args[1].parse::<String>().unwrap());
            let zarr_path: String =
                format!("{}/{}.zarr", FOLDER, args[1].parse::<String>().unwrap());
            let n_nodes: i128 = args[2].parse::<i128>().unwrap();
            let n_predicates: i128 = args[3].parse::<i128>().unwrap();
            let n_triples: i128 = args[4].parse::<i128>().unwrap();

            create_benchmark_database(file_path, zarr_path, n_nodes, n_predicates, n_triples);
        }
        _ => {
            panic!(
                "
                Usage: cargo run <n_nodes> <n_predicates> <n_triples>
                Usage: cargo run <file_name> <n_nodes> <n_predicates> <n_triples>
                "
            );
        }
    }
}

fn create_benchmark_database(
    file_path: String,
    zarr_path: String,
    n_nodes: i128,
    n_predicates: i128,
    n_triples: i128,
) {
    create_file_nt(&file_path, n_nodes, n_predicates, n_triples).unwrap();
    parse_to_zarr(&zarr_path, &file_path);
}

fn create_file_nt(
    file_path: &String,
    n_nodes: i128,
    n_predicates: i128,
    n_triples: i128,
) -> io::Result<()> {
    //prerequisites for creating the benchmar
    let name_generator = RNG::from(&Language::Elven);
    let mut rng = rand::thread_rng();
    let _ = fs::create_dir(FOLDER);
    let mut file = File::create(file_path)?;

    //Initializing the vectors
    let mut nodes: Vec<String> = vec![];
    let mut predicates: Vec<String> = vec![];

    //We create two vectors with the nodes and predicates
    for i in 0..n_nodes {
        nodes.push(format!(
            "{}-{}",
            name_generator.generate_name_by_count(4),
            i
        ))
    }
    for i in 0..n_predicates {
        predicates.push(format!(
            "{}-{}",
            name_generator.generate_name_by_count(0),
            i
        ))
    }

    //We create a random triple between two nodes
    for _ in 0..n_triples {
        let s_index = rng.gen_range(0..n_nodes) as usize;
        let p_index = rng.gen_range(0..n_predicates) as usize;
        let mut o_index = rng.gen_range(0..n_nodes) as usize;

        //Relations of a node to itself are not allowed
        while s_index == o_index {
            o_index = rng.gen_range(0..n_nodes) as usize;
        }

        let s = &nodes[s_index];
        let p = &predicates[p_index];
        let o = &nodes[o_index];
        writeln!(
            file,
            "<http://example.org/{}>    <http://example.org/{}>    <http://example.org/{}> .",
            s, p, o
        )?;

       
    }

    Ok(())
}

fn parse_to_zarr(zarr_path: &str, file_path: &str) {
    LocalStorage::new(TabularLayout)
        .serialize(zarr_path, file_path, ChunkingStrategy::Chunk)
        .unwrap();
}
