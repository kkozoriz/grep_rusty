use clap::Parser;
use grep_rusty::Args;
use std::process;
fn main() {
    let args = Args::parse();

    if let Err(e) = grep_rusty::run(args) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
