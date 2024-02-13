use rand::Rng;
use remote_hdt::storage::layout::tabular::TabularLayout;
use remote_hdt::storage::layout::matrix::MatrixLayout;
use remote_hdt::storage::Storage;
use remote_hdt::storage::params::{Backend, ChunkingStrategy, ReferenceSystem, Serialization};

use rnglib::{Language, RNG};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::vec::Vec;


const FOLDER_NT: &str = "../nt-files";
const FOLDER_ZARR: &str = "../zarr-files";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    //Different inputs
    match args.len() {
        5 => {
            //Usage: cargo run <benchmark_name> <n_nodes> <n_predicates> <n_triples>
            let file_path: String = format!("{}/{}.nt", FOLDER_NT, args[1].parse::<String>().unwrap());
            let n_nodes: i128 = args[2].parse::<i128>().unwrap();
            let n_predicates: i128 = args[3].parse::<i128>().unwrap();
            let n_triples: i128 = args[4].parse::<i128>().unwrap();
            let zarr_path_tabular: String =
                    format!("{}/{}-tabular.zarr", FOLDER_ZARR, args[1].parse::<String>().unwrap());
            let zarr_path_matrix: String =
                    format!("{}/{}-matrix.zarr", FOLDER_ZARR, args[1].parse::<String>().unwrap());
            

            create_file_nt(&file_path, n_nodes, n_predicates, n_triples).unwrap();

            parse_to_zarr_tabular_layout(&zarr_path_tabular, &file_path, &(1000 as u64));
            parse_to_zarr_matrix_layout(&zarr_path_matrix, &file_path, &(1000 as u64));
        },
        _ => {
            panic!(
                "
                Usage for creating nt files: cargo run <file_name> <n_nodes> <n_predicates> <n_triples>
                Usage for parsing nt files to zarr: cargo run <storage_strategy> 
                "
            );
        }
    }
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
    let _ = fs::create_dir(FOLDER_NT);
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

fn parse_to_zarr_tabular_layout(zarr_path: &str, file_path: &str, shard_size: &u64) {
    Storage::new(TabularLayout, Serialization::Zarr).serialize(
        Backend::FileSystem(zarr_path),
        file_path,
        ChunkingStrategy::Sharding(*shard_size),
        ReferenceSystem::SPO,
    ).unwrap();
}

fn parse_to_zarr_matrix_layout(zarr_path: &str, file_path: &str, shard_size: &u64) {
    Storage::new(MatrixLayout, Serialization::Zarr).serialize(
        Backend::FileSystem(zarr_path),
        file_path,
        ChunkingStrategy::Sharding(*shard_size),
        ReferenceSystem::SPO,
    ).unwrap();
}
