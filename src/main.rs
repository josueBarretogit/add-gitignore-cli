use std::process;
use crate::search_gitignore::search::*;
use  crate::generate_gitignore::generate::*;

use clap::Parser;

mod search_gitignore;
mod generate_gitignore;
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

    if !is_manual {
        match args.lang {
            Some(language) => {
                let search_result = search_for_gitignore(language).await;
                match search_result {
                    Ok(_) => println!("gitignore generated in root directory"),
                    Err(e) => {
                        eprintln!("{e}");

                        process::exit(1);

                    }
                }

            }
            None => {
                eprint!("A language was not provided");
                process::exit(1);
            }
        }
        return;
    }
    generate_gitignore_manually();
}

