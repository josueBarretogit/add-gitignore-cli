use crate::generate_gitignore::generate::*;
use crate::search_gitignore::search::*;
use std::path::Path;
use std::process;

use inquire::{Confirm, Select};

mod generate_gitignore;
mod search_gitignore;
mod utils;

#[tokio::main]
async fn main() {
    let lang_options = vec!["Rust", "Node", "Go", "Dart", "Java", "C", "Javascript"];

    if Path::new(".gitignore").exists() {
        let answer = Confirm::new("There is a .gitignore already, Do you want to overwrite it?")
            .with_default(false)
            .prompt();

        match answer {
            Ok(true) => {}
            Ok(false) => {
                println!("I will not touch your already existing .gitignore");
                process::exit(0)
            }
            Err(e) => {
                eprint!("There was an error, more details: {}", e);
                process::exit(1)
            }
        }
    }

    let answer = match Select::new("Choose the language to get .gitignore for", lang_options)
        .with_vim_mode(true)
        .prompt()
    {
        Ok(choice) => choice,
        Err(err) => {
            eprint!("There was an error, more details: {err}");
            process::exit(1)
        }
    };

    println!("Getting your gitignore ...");

    let search_result = search_for_gitignore(answer.to_string()).await;

    if let Err(error) = search_result {
        eprintln!("an error ocurred, details: {error}");
        process::exit(1);
    }
    println!(".gitignore generated in root dir");
    process::exit(0)
}
