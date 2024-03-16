use benchmark::BenchmarkExecutor;
use clap::Parser;
use clap::ValueEnum;
use remote_hdt::storage::layout::matrix::MatrixLayout;
use remote_hdt::storage::layout::tabular::TabularLayout;
use remote_hdt::storage::params::Backend;
use remote_hdt::storage::params::Serialization;

mod benchmark;
mod utils;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short)]
    iterations: u8,
    #[arg(short, num_args = 1..)]
    files: Vec<String>,
    #[arg(
        long,
        default_value_t = Context::Local,
        value_enum
    )]
    context: Context,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Context {
    Remote,
    Local,
}

fn main() {
    let args = Cli::parse();
    let benchmark_executor = BenchmarkExecutor::new(args.iterations);

    for file in args.files {
        let context = match args.context {
            Context::Remote => Backend::HTTP(&file),
            Context::Local => Backend::FileSystem(&file),
        };

        // TODO: fix this by modifying RemoteHDT
        if file.contains("matrix") {
            benchmark_executor.execute(MatrixLayout, Serialization::Zarr, &context)
        } else if file.contains("tabular") {
            benchmark_executor.execute(TabularLayout, Serialization::Sparse, &context)
        }
    }
}
