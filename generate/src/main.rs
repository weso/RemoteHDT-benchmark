
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::env;
use std::vec::Vec;
use rand::Rng;

const FILE_PATH: &str = "outputs/benchmark.nt";
const FOLDER: &str = "outputs";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        panic!("Usage: cargo run <n_subjects> <n_predicates> <n_objects> <n_triples>");
    }

    let n_subjects: i128 = args[1].parse::<i128>().unwrap();
    let n_predicates: i128 = args[2].parse::<i128>().unwrap();
    let n_objects: i128 = args[3].parse::<i128>().unwrap();
    let n_triples: i128 = args[4].parse::<i128>().unwrap();

    let _ = create_benchmark_2(n_subjects, n_predicates, n_objects, n_triples);
}

fn create_benchmark_2(n_subjects: i128 , n_predicates: i128, n_objects: i128, n_triples:i128) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let _ = fs::create_dir(FOLDER);
    let mut file = File::create(FILE_PATH)?;

    let subjects: Vec<i128> = (0..n_subjects).collect();
    let predicates: Vec<i128> = (1..n_predicates+1).collect();
    let objects: Vec<i128> = (0..n_objects).collect();


    for _ in 0..n_triples{
        let s = subjects[rng.gen_range(0..n_subjects) as usize];
        let p = predicates[rng.gen_range(0..n_predicates) as usize];
        let o= objects[rng.gen_range(0..n_objects) as usize];
        writeln!(file,"<http://example.org/{}>    <http://example.org/{}>    <http://example.org/{}> .", s, p, o)?;
    }

    Ok(())
}

