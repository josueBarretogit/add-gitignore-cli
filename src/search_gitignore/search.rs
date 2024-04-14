use  crate::utils::AnyError;
use reqwest::Url;
use tokio::{fs::File, io::AsyncWriteExt};


const GITHUB_REPO_URL: &str =
    "https://raw.githubusercontent.com/github/gitignore/main/";


async fn get_gitignore_contents(uri: Url) -> Result<String, AnyError> {
    let response = reqwest::get(uri).await;

    if let Err(error) = &response {
        
        if error.is_request() {

        } 
        

    }

    Ok(response.unwrap().text_with_charset("utf-8").await?)
}

 async fn build_gitignore(contents: &[u8]) -> Result<(), AnyError> {
    let mut new_gitignore = File::create(".gitignore").await?;

    new_gitignore
        .write_all(format!("#Fetched by: add-gitignore cli from: {} \n", GITHUB_REPO_URL).as_bytes())
        .await?;

    new_gitignore.write_all(contents).await?;

    Ok(())
}

pub async fn search_for_gitignore(lang: String) -> Result<(), AnyError> {

    let mut lang_to_search = lang.trim().to_owned();

    

    lang_to_search.push_str(".gitignore");

    let endpoint_gitignore = Url::parse(GITHUB_REPO_URL)?.join(&lang_to_search)?; 

    println!("{endpoint_gitignore}");

    let contents = get_gitignore_contents(endpoint_gitignore).await?;

    build_gitignore(contents.as_bytes()).await?;

    Ok(())
}








