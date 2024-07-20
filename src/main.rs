use std::env;
use reqwest::get;
use serde::{Serialize, Deserialize};
use skyscraper::html::{self, parse::ParseError};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryData {
  pub project: String,
  pub head: String,
  pub pending: String,
  pub version: String,
  pub created: String,
  pub languages: String,
  pub dependencies: String,
  pub authors: String,
  pub last_changes: String,
  pub contributors: String,
  pub repo: String,
  pub commits: String,
  pub lines_of_code: String,
  pub size: String,
  pub license: String,

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    panic!("NO REPOSITORY WAS PROVIDED");
  }

  let url = &args[1];
  let resp = get(url).await?.text().await?;
  let document = html::parse(&*resp);

  // res to string
  // string to html::parse
  // html to RepositoryData
  // repository data to imprimir x pantalla
  // let document = html::parse(html_text)?;

  println!("{:?}", args);

  Ok(())
}

