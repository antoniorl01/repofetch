mod get_repository_response;
mod get_repository_commits;
mod get_repository_languages;

use std::env;
use reqwest::get;
use serde::{Serialize, Deserialize};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args: Vec<String> = env::args().collect();
  args.push("https://api.github.com/repos/antoniorl01/repofetch".to_owned());

  if args.len() < 2 {
    panic!("NO REPOSITORY WAS PROVIDED");
  }

  let url = &args[1];
  let resp = get(url).await?;
  let repo_data: get_repository_response::Root = resp.json().await?;

  // problema, aqui cambian las propiedades
  let resp = get(&url.to_owned().push_str("/languages")).await?;

  let resp = get(&url.to_owned().push_str("/commits")).await?;
  let commits: get_repository_commits::Root = resp.json().await?;
  let commit_num = commits.len();

  Ok(())
}

// https://api.github.com/repos/antoniorl01/repofetch
// https://api.github.com/repos/antoniorl01/repofetch/languages
// https://api.github.com/repos/antoniorl01/repofetch/commits

