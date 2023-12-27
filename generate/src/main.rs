use rand::Rng;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::vec::Vec;
use rnglib::{RNG, Language};

const FILE_NAME: &str = "benchmark";

const FOLDER: &str = "outputs";

fn main() {
    let args: Vec<String> = env::args().collect();

    //Different inputs
    match args.len(){
        4 => {
            // Usage: cargo run <n_nodes> <n_predicates> <n_triples>
            let file_path: String = format!("{}/{}.nt", FOLDER,FILE_NAME);
            let n_nodes: i128 = args[1].parse::<i128>().unwrap();
            let n_predicates: i128 = args[2].parse::<i128>().unwrap();
            let n_triples: i128 = args[3].parse::<i128>().unwrap();

            let _ = create_benchmark_nt(file_path,n_nodes, n_predicates,  n_triples);
            
        },
        5 => {
            //Usage: cargo run <benchmarc_name> <n_nodes> <n_predicates> <n_triples>
            let file_path : String = format!("{}/{}.nt", FOLDER,args[1].parse::<String>().unwrap());
            let n_nodes: i128 = args[2].parse::<i128>().unwrap();
            let n_predicates: i128 = args[3].parse::<i128>().unwrap();
            let n_triples: i128 = args[4].parse::<i128>().unwrap();

            let _ = create_benchmark_nt(file_path,n_nodes, n_predicates,  n_triples);            
        },
        _ =>{
            panic!(
                "
                Usage: cargo run <n_nodes> <n_predicates> <n_triples>
                Usage: cargo run <file_name> <n_nodes> <n_predicates> <n_triples>
                "
            );
        }
    }

    
}

fn create_benchmark_nt(
    file_path: String,
    n_nodes: i128,
    n_predicates: i128,
    n_triples: i128,
) -> io::Result<()> {

    //prerequisites for creating the benchmarc
    let name_generator = RNG::try_from(&Language::Elven).unwrap();
    let mut rng = rand::thread_rng();
    let _ = fs::create_dir(FOLDER);
    let mut file = File::create(file_path)?;

    //Initializing the vectors
    let mut nodes: Vec<String> = vec![];
    let mut predicates: Vec<String> = vec![];


    //We create two vectors with the nodes and predicates
    for i in 0..n_nodes {
        nodes.push(format!("{}-{}", name_generator.generate_name_by_count(4), i))
    }
    for i in 0..n_predicates {
        predicates.push(format!("{}-{}", name_generator.generate_name_by_count(0), i))
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
