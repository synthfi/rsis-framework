
mod block_data;
mod generate;

use block_data::*;
use clap::Parser;
use generate::generate_interface;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use toml::Table;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    file: String,

    #[arg(short, long, default_value = "rust")]
    lang: String,

    #[arg(long)]
    verbose: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let temp = args.file.clone();
    let filepath = Path::new(&temp);
    if !filepath.exists() {
        println!("File: [{}] does not exist", args.file);
        return ExitCode::FAILURE;
    }
    let contents = fs::read_to_string(args.file)
        .expect("Failed to read file");
    let data = contents.parse::<Table>()
        .expect("Failed to parse TOML file");

    // do basic checking on the structure of the TOML file
    let db = match parse_interface(&data, args.verbose) {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("Failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // target generated file will have the same basename
    let basedir = match filepath.parent() {
        Some(parent) => parent.to_path_buf(),
        None => PathBuf::new()
    };
    let basename = basedir.join(db.interface.clone());
    match generate_interface(basename.as_path(), &db, args.lang) {
        Ok(()) => {},
        Err(e) => {
            println!("{}", e);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
