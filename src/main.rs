use clap::Parser;
use grep_rusty::{Args, GrepRusty};

fn main() {
    let args = Args::parse();

    GrepRusty::default()
        .set_config(&args)
        .run(&args.pattern, &args.file_path)
        .print_result(&args.pattern, &args.file_path);
}
