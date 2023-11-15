use clap::Parser;
use shikibetsu::{run_with_args, Args};

fn main() {
    let args = Args::parse();

    run_with_args(&args);
}
