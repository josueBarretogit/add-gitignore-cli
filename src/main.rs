use std::{error::Error, process};

use  clap::Parser;
use reqwest::Url;
use tokio::{fs::File, io::AsyncWriteExt};

const GITHUB_REPO_URL: &str = "https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore";

type AnyError = Box<dyn Error>;

#[derive(Parser , Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    manual : bool,
    #[arg(short, long)]
    lang : Option<String>
}



#[tokio::main]
async fn main() {
    let args = Args::parse();


    let is_manual = args.manual;

    if !is_manual {


        match args.lang {
            Some(language) => {
                search_for_gitignore(language).await;
                println!("gitignore generated in root directory");
            }
            None => { 
                eprint!("a language was not provided");
                process::exit(1);
            }
        }
        return;

    } 
    generate_gitignore_manually();
}

async fn get_gitignore_contents(uri : Url) -> Result< String,  AnyError>  {
    let response = reqwest::get(uri).await?.text_with_charset("utf-8").await?;

    Ok(response)
}


async fn build_gitinore(contents : &[u8]) -> Result<(), AnyError> {

    let mut new_gitignore = File::create(".gitignore").await?;

    new_gitignore.write_all("# Fetched by: add-gitignore cli \n".as_bytes()).await?;

    new_gitignore.write_all(contents).await?;

    Ok(())
}


async fn search_for_gitignore(lang : String) -> Result<(),  AnyError> {

    let contents = get_gitignore_contents(Url::parse(GITHUB_REPO_URL).unwrap()).await?;

    build_gitinore(contents.as_bytes()).await?;

    Ok(())
}


 fn generate_gitignore_manually() {
    println!("generating .gitignore ...");
}


