use clap::Parser;
use colored::Colorize;
use grep_rusty::Args;
use std::process;

fn main() {
    let args = Args::parse();

    match grep_rusty::run(&args) {
        Ok(result) => print_result(&args, result),
        Err(e) => {
            eprintln!("Application error {e}");
            process::exit(1);
        }
    }
}

fn print_result(args: &Args, results: Vec<String>) {
    if results.is_empty() {
        println!(
            "Query {} not found in file {}",
            args.pattern.red(),
            args.file_path.display().to_string().purple()
        );
    } else {
        println!("Found {} lines\nResult of search:", results.len());
        results.iter().for_each(|line| println!("{line}"))
    }
}
