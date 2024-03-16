
use remote_hdt::storage::layout::tabular::TabularLayout;
use remote_hdt::storage::layout::matrix::MatrixLayout;

use remote_hdt::storage::params::{Backend, Serialization};
use remote_hdt::storage::Storage;
use remote_hdt::storage::ops::Ops;


use std::time::Instant;
use std::time::Duration;

use std::env;

use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use serde_json::Value;

const DATABASE_FOLDER: &str = "../zarr-files";
const BENCHMARK_RESULTS_DESTINATION_FOLDER:  &str = "../results";
const BENCHMARK_RESULTS_DESTINATION_FILE_LOCAL:  &str = "../results/local_benchmark.csv";
const BENCHMARK_RESULTS_DESTINATION_FILE_REMOTE:  &str = "../results/remote_benchmark.csv";
const CSV_HEADER: &str = "file_name,get_subject,get_predicate,get_object";
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
        let zarr_path = &format!("{}/{}", DATABASE_FOLDER, file.clone());

        
            
            if file.contains("matrix"){

                for _ in 0..iterations {
                let mut binding = Storage::new(MatrixLayout, Serialization::Zarr);
                let arr = binding.load(Backend::FileSystem(&zarr_path.as_str())).unwrap();

                let results = execute_benchmark::<_>(&format!("{}/{}", DATABASE_FOLDER, file.clone()), arr);
                times.push((file.clone() , results));
            }

            }
        
        
            if file.contains("tabular"){
                for _ in 0..iterations {
                let mut binding = Storage::new(TabularLayout, Serialization::Zarr);
                let arr = binding.load(Backend::FileSystem(&zarr_path.as_str())).unwrap();

                let results = execute_benchmark::<_>(&format!("{}/{}", DATABASE_FOLDER, file.clone()), arr);
                times.push((file.clone() , results));
            }
            }
       
        
        
    }
    write_csv(times,BENCHMARK_RESULTS_DESTINATION_FILE_LOCAL);

}





fn remote_execution(iterations: u8, files:&Vec<String>){
    let mut times : Vec<(String, (Duration,Duration,Duration))> = vec![];

    for file in files {
        let zarr_path = &format!("{}/{}", DATABASE_URL, file.clone());
            if file.contains("matrix"){

                for _ in 0..iterations {
                let mut binding = Storage::new(MatrixLayout, Serialization::Zarr);

                let arr = binding.load(Backend::HTTP(&zarr_path.as_str())).unwrap();

                let results = execute_benchmark::<_>(&format!("{}/{}", DATABASE_FOLDER, file.clone()), arr);
                times.push((file.clone() , results));
            }

            }
        
        
            if file.contains("tabular"){
                for _ in 0..iterations {
                let mut binding = Storage::new(TabularLayout, Serialization::Zarr);
                let arr = binding.load(Backend::FileSystem(&zarr_path.as_str())).unwrap();

                let results = execute_benchmark::<_>(&format!("{}/{}", DATABASE_FOLDER, file.clone()), arr);
                times.push((file.clone() , results));
            }
            }
       
        
        
    }
    write_csv(times,BENCHMARK_RESULTS_DESTINATION_FILE_REMOTE);
}

/**
 * commented lines are for when the get predicate operation is implemented
 */
fn execute_benchmark<T>(zarr_path: &str, arr:&Storage<T>) -> (Duration,Duration,Duration){
    let first_term = execute_get_subject( arr, get_subject_zarr(zarr_path).as_str());
    let second_term = execute_get_predicate( arr, get_predicate_zarr(zarr_path).as_str());
    let third_term = execute_get_object( arr, get_object(zarr_path).as_str());
    (first_term,second_term,third_term)
}   




//---------------------------- Operations -------------------------
fn execute_get_subject<T>(  arr:&Storage<T>, first_term:&str )-> Duration{

    let before = Instant::now();
    let _ = arr.get_subject(first_term);
    before.elapsed()
}

fn execute_get_predicate<T>(  arr:&Storage<T>, first_term:&str )-> Duration{
    
    let before = Instant::now();
    let _ = arr.get_predicate(first_term);
    before.elapsed()
}

fn execute_get_object<T>(  arr:&Storage<T>, first_term:&str )-> Duration{

    let before = Instant::now();
    let _ = arr.get_object(first_term);
    before.elapsed()
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

fn get_node_zarr(zarr_path: &str) -> String{
    let mut file = File::open(format!("{}/{}", zarr_path,"group/RemoteHDT/zarr.json")).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let json: Value = serde_json::from_str(&contents).expect("Unable to parse JSON");
    

    
    let json_str = serde_json::to_string(&json["attributes"]["subjects"][0]).expect("Unable to convert JSON to string");
   
   


    // Find the index of the first < and the last >
    let start_index = json_str.find('<').unwrap_or(0);
    let end_index = json_str.rfind('>').unwrap_or(json_str.len());
 

    // Extract the substring between the < and >
    let result = json_str[start_index..end_index + 1].to_string();

    result
   
}

#[warn(dead_code)]
fn get_predicate_zarr(zarr_path: &str) -> String{

    let mut file = File::open(format!("{}/{}", zarr_path,"group/RemoteHDT/zarr.json")).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let json: Value = serde_json::from_str(&contents).expect("Unable to parse JSON");
    

    
    let json_str = serde_json::to_string(&json["attributes"]["predicates"][0]).expect("Unable to convert JSON to string");
   
   


    // Find the index of the first < and the last >
    let start_index = json_str.find('<').unwrap_or(0);
    let end_index = json_str.rfind('>').unwrap_or(json_str.len());
 

    // Extract the substring between the < and >
    let result = json_str[start_index..end_index + 1].to_string();

    result
   

}
