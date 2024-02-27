use clap::Parser;
use std::fmt::{Debug, Formatter};
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
};

/* Do not modify this */
// CLI args parser
#[derive(Parser, Debug)]
pub struct Args {
    // Csv file read path
    #[arg(short, long)]
    pub read_path: PathBuf,
    // Sub command for handling data in csv file
    #[clap(subcommand)]
    pub command: Command,
}

/* Do not modify this */
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
        #[clap(short, long)]
        data: String,
    },
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct CSVFile {
    data: Vec<Vec<String>>,
    rows: usize,
    cols: usize,
}

pub trait CSVFileReader {
    fn read(&mut self, file_path: &Path) -> Result<(), Error>;
}

pub trait CSVFileSaver {
    fn save(&self, file_path: &Path) -> Result<(), Error>;
}

pub trait CSVFileEditor {
    fn replace(&mut self, row: usize, col: usize, data: String) -> Result<(), Error>;
}

impl std::fmt::Display for CSVFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            for (i, col) in row.iter().enumerate() {
                if i == 0 {
                    f.write_fmt(format_args!("{col}"))?;
                } else {
                    f.write_fmt(format_args!(",{col}"))?;
                }
            }

            if i + 1 != self.rows {
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

impl CSVFileReader for CSVFile {
    fn read(&mut self, file_path: &Path) -> Result<(), Error> {
        let file = File::open(file_path)?;
        let buff = BufReader::new(file);

        let mut rows = 0;
        let mut columns = 0;
        let mut data = Vec::new();

        for (index, line) in buff.lines().enumerate() {
            rows += 1;
            let line = line?;
            data.push(Vec::new());

            if index == 0 {
                for col in line.split(',') {
                    columns += 1;
                    data.last_mut().unwrap().push(col.to_string());
                }
            } else {
                for col in line.split(',') {
                    data.last_mut().unwrap().push(col.to_string());
                }

                if data.last().unwrap().len() != columns {
                    return Err(Error::new(ErrorKind::InvalidInput, "inconsistent CSV"));
                }
            }
        }

        self.rows = rows;
        self.cols = columns;
        self.data = data;

        Ok(())
    }
}

impl CSVFileEditor for CSVFile {
    fn replace(&mut self, row_idx: usize, col_idx: usize, data: String) -> Result<(), Error> {
        if let Some(row) = self.data.get_mut(row_idx) {
            if let Some(val) = row.get_mut(col_idx) {
                *val = data;
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("col {col_idx} doesn't exist"),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("row {row_idx} doesn't exist"),
            ))
        }
    }
}

impl CSVFileSaver for CSVFile {
    fn save(&self, file_path: &Path) -> Result<(), Error> {
        let mut file = File::open(file_path)?;
        file.write_fmt(format_args!("{self}"))?;

        Ok(())
    }
}

fn handle_error<T, E: std::error::Error + Debug>(res: Result<T, E>, error_decr: Option<&str>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            println!("{}: {e:#?}", error_decr.unwrap_or("<unknown>"));
            std::process::exit(1);
        }
    }
}

fn main() {
    // Reading CLI args
    let args = Args::parse();

    // create CSVFile instance and read file into it
    let mut csv = CSVFile::default();

    handle_error(csv.read(&args.read_path), Some("failed reading csv"));

    // match and execute command
    match args.command {
        Command::Display => println!("{csv}"),
        Command::Replace { row, col, data } => {
            handle_error(csv.replace(row, col, data), Some("failed editing csv"));
            handle_error(csv.save(&args.read_path), Some("failed saving csv"));
        }
    }
}
