use std::fs::create_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::benchmark::Line;

const BENCHMARK_RESULTS_DESTINATION_FOLDER: &str = "../results";
const CSV_HEADER: &str = "layout,file_name,get_subject,get_predicate,get_object";

pub fn write_csv(times: Vec<Line>) {
    if !Path::new(&BENCHMARK_RESULTS_DESTINATION_FOLDER).exists() {
        let _ = create_dir(&BENCHMARK_RESULTS_DESTINATION_FOLDER);
    }

    let path = format!("{}/benchmarks.csv", BENCHMARK_RESULTS_DESTINATION_FOLDER);
    let mut file = if Path::new(&path).exists() {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap()
    } else {
        let mut ans = File::create(path).unwrap();
        writeln!(ans, "{}", CSV_HEADER).unwrap();
        ans
    };

    for time in times {
        writeln!(
            file,
            "{},{},{},{:?},{:?},{:?}",
            time.layout,
            time.file_name,
            time.serialization,
            time.subject,
            time.predicate,
            time.object
        )
        .unwrap();
    }
}
