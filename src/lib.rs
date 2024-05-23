use clap::Parser;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub query: String,

    #[arg(short, long)]
    pub file_path: PathBuf,

    #[arg(short, long, default_value_t = true)]
    pub ignore_case: bool,
}

impl Args {
    pub fn print_result(&self, results: Vec<String>) {
        if results.is_empty() {
            println!(
                "Query {} not found in file {}",
                self.query,
                self.file_path.display()
            );
        } else {
            println!("Found {} lines\nResult of search:", results.len());
            results.iter().for_each(|line| println!("{line}"))
        }
    }
}

pub fn read_lines(file_path: &PathBuf) -> io::Result<impl ParallelIterator<Item = String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map_while(Result::ok).par_bridge())
}

fn search_query<T>(lines: T, query: &str) -> Result<Vec<String>, Box<dyn Error>>
where
    T: IntoParallelIterator<Item = String>,
{
    Ok(lines
        .into_par_iter()
        .filter(|line| line.contains(query))
        .collect())
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let reader_result = read_lines(&args.file_path)?;
    let search_result = search_query(reader_result, &args.query)?;

    args.print_result(search_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{read_lines, search_query};
    use std::path::PathBuf;

    #[test]
    fn search_query_test() {
        let query = "Except";
        let file_path = PathBuf::from("example.txt");
        let lines = read_lines(&file_path).unwrap();

        let result = search_query(lines, query).unwrap();

        assert_eq!(
            result,
            vec!["Except the Will which says to them: ‘Hold on!’".to_string()]
        )
    }
}
