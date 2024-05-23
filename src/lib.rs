use clap::Parser;
use rayon::prelude::*;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "grep-rusty")]
#[command(version = "0.1.0")]
#[command(about = "Command line arguments for grep_rusty", long_about = None)]
pub struct Args {
    /// The query string to search for
    pub pattern: String,

    /// The path to the file to search in
    pub file_path: PathBuf,

    /// Ignore case while searching
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Selected lines are those not matching any of the specified patterns
    #[arg(short = 'v', long = "invert-match")]
    pub invert_match: bool,
}

impl Args {
    /// Prints the result of the search
    ///
    /// # Arguments
    ///
    /// * `results` - A vector of strings containing the lines that match the query
    pub fn print_result(&self, results: Vec<String>) {
        if results.is_empty() {
            println!(
                "Query {} not found in file {}",
                self.pattern,
                self.file_path.display()
            );
        } else {
            println!("Found {} lines\nResult of search:", results.len());
            results.iter().for_each(|line| println!("{line}"))
        }
    }
}

/// Reads lines from a file and returns a parallel iterator over the lines
///
/// # Arguments
///
/// * `file_path` - The path to the file to read from
///
/// # Returns
///
/// * `io::Result<impl ParallelIterator<Item = String>>` - A result containing a parallel iterator over the lines in the file
pub fn read_lines(file_path: &PathBuf) -> io::Result<impl ParallelIterator<Item = String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map_while(Result::ok).par_bridge())
}

/// Searches for a query in a collection of lines
///
/// # Arguments
///
/// * `lines` - A collection of lines to search through
/// * `query` - The query string to search for
/// * `ignore_case` - Whether to ignore case in the search
///
/// # Returns
///
/// * `Result<Vec<String>, Box<dyn Error>>` - A result containing a vector of matching lines or an error
fn search_query<T>(lines: T, query: &str, ignore_case: bool) -> Result<Vec<String>, Box<dyn Error>>
where
    T: IntoParallelIterator<Item = String>,
{
    Ok(lines
        .into_par_iter()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase().contains(&query.to_lowercase())
            } else {
                line.contains(query)
            }
        })
        .collect())
}

/// Runs the grep_rusty utility
///
/// # Arguments
///
/// * `args` - The command line arguments
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or an error
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let reader_result = read_lines(&args.file_path)?;
    let search_result = search_query(reader_result, &args.pattern, args.ignore_case)?;

    args.print_result(search_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{read_lines, search_query, Args};
    use std::path::PathBuf;

    #[test]
    fn search_test() {
        let query = "Except";
        let file_path = PathBuf::from("example.txt");

        assert_eq!(
            search_query(read_lines(&file_path).unwrap(), query, true).unwrap(),
            vec![
                "Except the Will which says to them: ‘Hold on!’".to_string(),
                "except the Will which says to them: ‘Hold on!’".to_string(),
            ]
        );
        assert_eq!(
            search_query(read_lines(&file_path).unwrap(), query, false).unwrap(),
            vec!["Except the Will which says to them: ‘Hold on!’".to_string()]
        )
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
