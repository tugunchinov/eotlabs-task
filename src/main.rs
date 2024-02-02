//! # Requirements
//! display(): Prints out the csv struct
//! replace(): Replaces an item in the csv struct by another at any given coordinates
//!
//! # Some other Requirements
//! Please utilize the following in your solution:- Associate Types, Traits, Trait Constraints, Associate Types, Error<>, Option<>
//! Custom error variants and handle them where possible.
//! Please pay attention to code readability and structure - separate into multiple files if possible.
//!
//! ## NOTE
//! Use only std and the crates provided
//!
//! # Demo Commands
//!
//! ## Display
//! cargo run -- --read-path=./data.csv display
//!
//! ## Replace
//! cargo run -- --read-path=./data.csv --write-path=./write.csv replace -r 1 -c 1 -d yolo

use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

// CLI args parser
#[derive(Parser, Debug)]
pub struct Args {
    // Csv file read path
    #[arg(short, long)]
    pub read_path: PathBuf,
    // Output data to new csv file or update existing one
    #[arg(short, long)]
    pub write_path: Option<PathBuf>,
    // Sub command for handling data in csv file
    #[clap(subcommand)]
    pub command: Command,
}

// Command and args prser
#[derive(Parser, Debug)]
pub enum Command {
    // Display entire file
    Display,
    // Modify a row/field
    Replace {
        #[clap(short, long)]
        row: usize,

        #[clap(short, long)]
        col: usize,

        // the new item to put into csv file
        #[clap(short, long, value_delimiter = ',')]
        data: String,
    },
}

// TODO Some custom errors you will implement
#[derive(Debug)]
enum CSVError {
    SomeError = 0,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct CSVData {
    data: Vec<Vec<String>>,
    rows: usize,
    cols: usize,
}

impl CSVData {
    // TODO read data into CSV struct
    pub fn read(&self, file_path: PathBuf) {
        println!("TODO: Reads CSV file into CSV struct")
    }

    // TODO replaces an item in give coordinates
    pub fn replace(&self, row: usize, col: usize, data: String) {
        println!(
            "TODO: Replace item at coords ({:?},{:?}) by ({:?})",
            row, col, data
        )
    }

    // TODO display this file
    pub fn display(&self) {
        println!("TODO: Prints the CSV data")
    }
}

fn main() {
    // Reading CLI args
    let args = Args::parse();

    // create CSVData instance and read file into it
    let mut csv = CSVData::default();
    csv.read(args.read_path);

    // match and execute command
    match args.command {
        Command::Display => csv.display(),
        Command::Replace { row, col, data } => {
            csv.replace(row, col, data);
            csv.display()
        }
    }
}
