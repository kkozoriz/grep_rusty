//! [![github]](https://github.com/kkozoriz/grep_rusty.git)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! `grep_rusty` is a simple command line utility for searching patterns in files.
//!
//! It provides functionality to search for a given pattern in text files, with options to perform
//! case-sensitive or case-insensitive searches, and to invert the match results. The utility
//! leverages parallel processing for efficient searching of large files.
//!
//! ## Usage
//!
//! The utility can be invoked from the command line with the following syntax:
//!
//! ```shell
//! grep-rusty [OPTIONS] <PATTERN> <FILE>
//! ```
//!
//! Where:
//!
//! - `<PATTERN>`: The pattern to search for in the file.
//! - `<FILE>`: The path to the file in which to search for the pattern.
//!
//! The following options are available:
//!
//! - `-i, --ignore-case`: Perform a case-insensitive search.
//! - `-v, --invert-match`: Invert the match results, selecting lines that do not match the pattern.
//!
//! ## Examples
//!
//! Perform a case-insensitive search for the pattern "error" in the file "log.txt":
//!
//! ```shell
//! grep-rusty -i error log.txt
//! ```
//!
//! Perform a case-sensitive search for the pattern "warning" in the file "log.txt", selecting lines
//! that do not match the pattern:
//!
//! ```shell
//! grep-rusty -v warning log.txt
//! ```

mod search;

use clap::Parser;
use rayon::prelude::*;
pub use search::*;

use colored::Colorize;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process;

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

    #[arg(short = 'w', long = "word-regexp")]
    pub word_regexp: bool,

    /// Selected lines are those not matching any of the specified patterns
    #[arg(short = 'v', long = "invert-match")]
    pub invert_match: bool,
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
/// * `config` - The search configuration
///
/// # Returns
///
/// * `Result<Vec<String>, Box<dyn Error>>` - A result containing a vector of matching lines or an error
fn search_query<T>(
    lines: T,
    query: &str,
    config: &SearchConfig,
) -> Result<Vec<String>, Box<dyn Error>>
where
    T: IntoParallelIterator<Item = String>,
{
    Ok(lines
        .into_par_iter()
        .filter(|line| config.matches(line, query))
        .collect())
}

pub struct GrepRusty {
    search_config: SearchConfig,
    result: Result<Vec<String>, Box<dyn Error>>,
}

impl GrepRusty {
    pub fn new() -> GrepRusty {
        GrepRusty {
            search_config: Default::default(),
            result: Ok(vec![]),
        }
    }
    pub fn set_config(mut self, args: &Args) -> GrepRusty {
        let mut config = SearchConfig::default();

        if args.ignore_case {
            config.add_config(Box::new(CaseInsensitive))
        } else {
            config.add_config(Box::new(CaseSensitive))
        }

        if args.word_regexp {
            config.add_config(Box::new(WordRegExp {
                case_insensitive: args.ignore_case,
            }))
        }

        if args.invert_match {
            config = SearchConfig {
                configs: vec![Box::new(InvertMatch { inner: config })],
            };
        }

        self.search_config = config;

        self
    }

    pub fn run(mut self, pattern: &str, file_path: &PathBuf) -> GrepRusty {
        match read_lines(file_path) {
            Ok(reader) => {
                self.result = search_query(reader, pattern, &self.search_config);
                self
            }
            Err(e) => {
                eprintln!("Application error {e}");
                process::exit(1);
            }
        }
    }

    pub fn print_result(self, pattern: &str, file_path: &Path) {
        if let Ok(result) = self.result {
            if result.is_empty() {
                println!(
                    "Query {} not found in file {}",
                    pattern.red(),
                    file_path.display().to_string().purple()
                );
            } else {
                println!("Found {} lines\nResult of search:", result.len());

                result.iter().for_each(|line| println!("{line}"))
            }
        }
    }
}

impl Default for GrepRusty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Args;

    #[test]
    fn search_test() {}

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
