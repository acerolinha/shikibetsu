use clap::Parser;
use shikibetsu::Args;

fn main() {
    let args = Args::parse();

    shikibetsu::run_with_args(&args);
}
