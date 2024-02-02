use clap::Parser;
use std::path::PathBuf;

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
    Modify {
        #[clap(short, long)]
        row: usize,

        #[clap(short, long)]
        col: usize,

        // the new item to put into csv file
        #[clap(short, long, value_delimiter = ',')]
        data: String,
    },
}
