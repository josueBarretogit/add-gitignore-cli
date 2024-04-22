use crate::generate_gitignore::generate::*;
use crate::search_gitignore::search::*;
use std::fs::{self, File};
use std::path::Path;
use std::process;

use clap::Parser;

mod generate_gitignore;
mod search_gitignore;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    manual: bool,
    #[arg(short, long)]
    lang: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let is_manual = args.manual;

    if Path::new(".gitignore").exists() {
        println!("a .gitignore already exists in the current directory");
        process::exit(1);
    }

    if !is_manual {
        if let None = args.lang {
            eprint!("A language was not provided");
            process::exit(1);
        }
        println!("Getting your gitignore ...");

        let search_result = search_for_gitignore(args.lang.unwrap()).await;

        if let Err(error) = search_result {
            eprintln!("an error ocurred, details: {error}");
            process::exit(1);
        }
        println!(".gitignore generated in root dir");
        process::exit(0)
    }
    generate_gitignore_manually();
}
