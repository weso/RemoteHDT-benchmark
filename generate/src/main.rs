

use std::fs::File;
use std::io::{self, Write};
use std::env;
use rand::Rng;

const FILE_PATH: &str = "benchmark.nt";

fn main() {
    let args: Vec<String> = env::args().collect();
    //Parameters n_subjects n_predicates n_objects 
    
    let n_subjects: i32 = args[1].parse::<i32>().unwrap();
    let n_predicates: i32 = args[2].parse::<i32>().unwrap();
    let n_objects: i32 = args[3].parse::<i32>().unwrap();
    let probability: f64 = args[4].parse::<f64>().unwrap();

    let _ = create_benchmark(n_subjects, n_predicates, n_objects, probability);
    
}


fn create_benchmark(n_subjects: i32 , n_predicates: i32, n_objects: i32, probability:f64) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut file = File::create(FILE_PATH)?;

    for s in 0..n_subjects {
        for p in 1..n_predicates+1{
            for o in 0..n_objects{
                if (rng.gen_range(1..=100) as f64 / 100.0) < probability{
                    writeln!(file,"<http://example.org/{}>    <http://example.org/{}>    <http://example.org/{}> .", s, p, o)?;
                }
            }
        }
    }
    
    Ok(())
}
