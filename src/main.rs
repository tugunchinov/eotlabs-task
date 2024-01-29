use anyhow::{bail, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};
use thiserror::Error;

// CLI args parser
#[derive(Parser, Debug)]
struct Args {
    // Csv file read path
    #[arg(short, long)]
    read_path: PathBuf,
    // Output data to new csv file or update existing one
    #[arg(short, long)]
    write_path: Option<PathBuf>,
    // Sub command for handling data in csv file
    #[clap(subcommand)]
    command: Command,
}

// Command and args prser
#[derive(Parser, Debug)]
enum Command {
    // Display entire file
    Display,
    // Modify a row/field
    Modify {
        #[clap(short, long)]
        row: usize,

        #[clap(short, long)]
        col: usize,

        // comma seperated values from cli
        #[clap(short, long, value_delimiter = ',')]
        data: Vec<String>,
    },
}

//Custom errors
#[derive(Error, Debug)]
enum Error {
    #[error("Some custom errors")]
    SomeErrors,
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
    pub fn from_file(file_path: PathBuf) -> Result<Self> {
        Ok(Self::default())
    }

    // TODO save CSV struct data to a file
    fn to_file(&self, file_path: PathBuf) -> Result<()> {
        Ok(())
    }
}
fn main() {
    // TODO Read and parse CLI args
    let args = Args::parse();
    println!("{:?}", args);
    // TODO read CSV file
    let csv_data = CSVData::from_file(args.read_path).unwrap();

    // TODO parse 
}
