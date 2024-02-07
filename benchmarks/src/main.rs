
use remote_hdt::engine::EngineStrategy;
use remote_hdt::storage::tabular::TabularLayout;
use remote_hdt::storage::HTTPStorage;
use remote_hdt::storage::LocalStorage;


use std::time::Instant;
use std::time::Duration;

use std::env;

use std::fs;
use std::fs::File;
use std::io::Write;

const DATABASE_FOLDER: &str = "../zarr-files";
const BENCHMARK_RESULTS_DESTINATION_FOLDER:  &str = "../results";
const BENCHMARK_RESULTS_DESTINATION_FILE_LOCAL:  &str = "../results/local_benchmark.csv";
const BENCHMARK_RESULTS_DESTINATION_FILE_REMOTE:  &str = "../results/remote_benchmark.csv";
const CSV_HEADER: &str = "file_name,get_subject_time,get_predicate_time,get_object_time";
const DATABASE_URL: &str = "http://localhost:8080";

fn main() {


    let args: Vec<String> = env::args().collect();

    if &args.len() != &(3 as usize){
        panic!("
            Usage cargo run <type_of_benchmark> <number_of_iterations>
            type_of_benchmark = local || remote || all
        ")

    }
   
    let type_of_benchmark = args[1].parse::<String>().unwrap();
    let iterations: u8 = (&args[2]).parse::<u8>().unwrap();
    let zarr_files = &get_dot_zarr_files();

    match type_of_benchmark.as_str() {
        "local" =>{
            local_execution(iterations,zarr_files);
        },
        "remote" =>{
            remote_execution(iterations,zarr_files);
        },
        "all" =>{
            execute_all(iterations, zarr_files);
        },
        _ =>{
            panic!("NOT A PARAMETER")
        }
    }


}


fn execute_all(iterations: u8, files:&Vec<String>){
    local_execution(iterations,&files);
    remote_execution(iterations, &files);   
}


fn local_execution(iterations: u8, files:&Vec<String>){
    
    let mut times : Vec<(String, (Duration,Duration,Duration))> = vec![];

    for file in files {
        for _ in 0..iterations {
            let benchmark_result = execute_local_benchmarks(&format!("{}/{}", DATABASE_FOLDER, file.clone()));

            times.push((file.clone() , benchmark_result));
        }
        
        
    }
    write_csv(times,BENCHMARK_RESULTS_DESTINATION_FILE_LOCAL);

}



fn remote_execution(iterations: u8, files:&Vec<String>){
    
    let mut times : Vec<(String, (Duration,Duration,Duration))> = vec![];

    for file in files {
        for _ in 0..iterations {
            let benchmark_result = execute_remote_benchmarks(&format!("{}/{}", DATABASE_URL, file.clone()));

            times.push((file.clone() , benchmark_result));
        }
        
        
    }
    write_csv(times,BENCHMARK_RESULTS_DESTINATION_FILE_REMOTE);

}

/**
 * commented lines are for when the get predicate operation is implemented
 */
fn execute_local_benchmarks(zarr_path: &str) -> (Duration,Duration,Duration){
    let subject_time = execute_subject_time(zarr_path);
    //let predicate_time = execute_predicate_time(zarr_path);
    let object_time = execute_object_time(zarr_path);
    (subject_time,Duration::new(0, 0),object_time)
}   

fn execute_remote_benchmarks(zarr_url: &str) -> (Duration,Duration,Duration){
    let subject_time = execute_subject_time_remote(zarr_url);
    //let predicate_time = execute_predicate_time_remote(zarr_url);
    let object_time = execute_object_time_remote(zarr_url);
    (subject_time,Duration::new(0, 0),object_time)
}   


//---------------------------- Operations -------------------------

fn execute_subject_time(zarr_path: &str) -> std::time::Duration{
    let database = LocalStorage::new(TabularLayout).load(zarr_path).unwrap();

    let before = Instant::now();
    let _ = database.get_subject(0);
    let subject_time = before.elapsed();

    subject_time
}

fn execute_subject_time_remote(zarr_url: &str) -> std::time::Duration{
    let database = HTTPStorage::new(TabularLayout).connect(zarr_url).unwrap();
    let before = Instant::now();
    let _ = database.get_subject(0);
    let subject_time = before.elapsed();

    subject_time
}



fn execute_predicate_time(zarr_path: &str) -> std::time::Duration{
    let database = LocalStorage::new(TabularLayout).load(zarr_path).unwrap();

    let before = Instant::now();
    let _ = database.get_predicate(0);
    let predicate_time = before.elapsed();

    predicate_time
}

fn execute_predicate_time_remote(zarr_url: &str) -> std::time::Duration{
    let database = HTTPStorage::new(TabularLayout).connect(zarr_url).unwrap();

    let before = Instant::now();
    let _ = database.get_predicate(0);
    let predicate_time = before.elapsed();

    predicate_time
}

fn execute_object_time(zarr_path: &str) -> std::time::Duration{
    let database = LocalStorage::new(TabularLayout).load(zarr_path).unwrap();

    let before = Instant::now();
    let _ = database.get_object(0);
    let object_time = before.elapsed();

    object_time
}


fn execute_object_time_remote(zarr_url: &str) -> std::time::Duration{
    let database = HTTPStorage::new(TabularLayout).connect(zarr_url).unwrap();

    let before = Instant::now();
    let _ = database.get_object(0);
    let object_time = before.elapsed();

    object_time
}


//----------------------------- Utils for the execution --------------------------



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


fn write_csv(times: Vec<(String, (Duration, Duration, Duration))>, destination_file: &str){

    if !folder_exists(&BENCHMARK_RESULTS_DESTINATION_FOLDER) {
        create_folder(&BENCHMARK_RESULTS_DESTINATION_FOLDER);
    }
    
    let mut file = File::create(destination_file).unwrap();
    
    writeln!(file, "{}", CSV_HEADER).unwrap();

    for time in times {
        writeln!(file, "{},{:?},{:?},{:?}", time.0, time.1.0, time.1.1, time.1.2).unwrap();
    }
        
    
}

fn folder_exists(folder_path: &str) -> bool {
    fs::metadata(folder_path).is_ok()
}

fn create_folder(folder_path: &str) {
    match fs::create_dir(folder_path) {
        Ok(_) => println!("Folder created successfully."),
        Err(e) => eprintln!("Error creating folder: {}", e),
    }
}