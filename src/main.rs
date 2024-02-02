use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

pub mod command;
use command::*;

// Some custom errors yo you will implement
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
    pub fn read(file_path: PathBuf) -> Result<(), CSVError> {
        Ok(())
    }

    // TODO Either update the existing csv file or create a new one - your choice
    pub fn write(&self, file_path: PathBuf) -> Result<(), CSVError> {
        Ok(())
    }

    // TODO display this file
    pub fn display(&self) {}
}

fn main() {
    // Reading CLI args and file
    let args = Args::parse();
    let mut csv_data = CSVData::read(args.read_path).unwrap();

    // match and execute command
    match args.command {
        Command::Display => println!("Prints the CSV data"),
        Command::Modify { row, col, data } => println!("Modifies the CSV file"),
    }
}
