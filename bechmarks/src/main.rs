
use remote_hdt::engine::EngineStrategy;
use remote_hdt::storage::tabular::TabularLayout;
use remote_hdt::storage::LocalStorage;

use std::time::Instant;
use std::time::Duration;

use std::env;

use std::fs;
use std::fs::File;
use std::io::Write;

const DATABASE_FOLDER: &str = "../zarr-files";
const BENCHMARK_RESULTS_DESTINATION_FILE:  &str = "../results/benchmark.csv";
const CSV_HEADER: &str = "file_name,get_subject_time,get_object_time,get_predicate_time";

fn main() {
    let args: Vec<String> = env::args().collect();


    match args.len() {
        1 =>{
            let iterations: u8 = args[1].parse::<u8>().unwrap();
            execute_all(iterations);
        },
        _ =>{
            execute_all(3);
        }
    }
}


fn execute_all(iterations: u8){
    let files = get_dot_zarr_files();
    let mut times : Vec<(String, (Duration,Duration,Duration))> = vec![];

    for file in files {
        for _ in 0..iterations {
            let benchmark_result = execute_benchmarks_with_zarr_rdf_database(&format!("{}/{}", DATABASE_FOLDER, file.clone()));
            times.push((file.clone() , benchmark_result));
        }
        
    }
    
    write_csv(times);
    
}


fn get_dot_zarr_files() -> Vec<String>{
    let paths = fs::read_dir(DATABASE_FOLDER).unwrap();

    let mut dot_zarr_files: Vec<String> = vec![];

    for path in paths {
        let path_name = path.unwrap().file_name().into_string().unwrap();
        if path_name.contains(".zarr") {
            dot_zarr_files.push(path_name);
        }
    }

   dot_zarr_files
}

/**
 * commented lines are for when the get predicate operation is implemented
 */
fn execute_benchmarks_with_zarr_rdf_database(zarr_path: &str) -> (Duration,Duration,Duration){
    let subject_time = execute_subject_time(zarr_path);
//    let predicate_time = execute_predicate_time(zarr_path);
    let object_time = execute_object_time(zarr_path);
//    (subject_time,predicate_time,object_time)
      (subject_time,Duration::new(0, 0),object_time)
}   

fn execute_subject_time(zarr_path: &str) -> std::time::Duration{
    let database = LocalStorage::new(TabularLayout).load(zarr_path).unwrap();

    let before = Instant::now();
    let _ = database.get_subject(0);
    let subject_time = before.elapsed();

    subject_time
}

/** 
fn execute_predicate_time(zarr_path: &str) -> std::time::Duration{
    let database = LocalStorage::new(TabularLayout).load(zarr_path).unwrap();

    let before = Instant::now();
    let _ = database.get_predicate(0);
    let predicate_time = before.elapsed();

    predicate_time
}
*/
fn execute_object_time(zarr_path: &str) -> std::time::Duration{
    let database = LocalStorage::new(TabularLayout).load(zarr_path).unwrap();

    let before = Instant::now();
    let _ = database.get_object(0);
    let object_time = before.elapsed();

    object_time
}

fn write_csv(times: Vec<(String, (Duration, Duration, Duration))>){
    
    let mut file = File::create(BENCHMARK_RESULTS_DESTINATION_FILE).unwrap();
    
    writeln!(file, "{}", CSV_HEADER).unwrap();

    for time in times {
        writeln!(file, "{},{:?},{:?},{:?}", time.0, time.1.0, time.1.1, time.1.2).unwrap();
    }
        
    
}

