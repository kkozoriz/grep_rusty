use clap::Parser;
use grep_rusty::Args;
use std::{process, time};
fn main() {
    let start = time::Instant::now();
    let args = Args::parse();

    if let Err(e) = grep_rusty::run(args) {
        eprintln!("Application error {e}");
        process::exit(1);
    }

    let end = start.elapsed();
    println!("Elapsed: {:.2?}", end);
}
