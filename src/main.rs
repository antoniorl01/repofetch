mod get_repository_response;
mod get_repository_commits;
mod get_repository_languages;

use std::fmt::Debug;

//use std::env;
//use reqwest::get;
//use serde::{Serialize, Deserialize};
use colored::Colorize;


  /*

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
   */

// https://api.github.com/repos/antoniorl01/repofetch
// https://api.github.com/repos/antoniorl01/repofetch/languages
// https://api.github.com/repos/antoniorl01/repofetch/commits


// https://github.com/forhappy/Cplusplus-Concurrency-In-Practice

enum Language {
  CSharp, 
  CPlusPlus,
  C,
  Java,
  Javascript,
  Typescript,
  Go,
  Rust,
  Scala,
  Haskell,
  Cobol,
  Ruby,
}


fn print_language_logo(language: Language) {
  match language {
          Language::C => {
            let logo = 
 "
             +++++++++++++++++
           +++++++++++++++++++++++
       ++++++++++++++++++++++++++++
     +++++++++++++++++++++++++++++++++
   ++++++++++++*************++++++++++++
 ++++++++++++*******************++++++::::
 ++++++++++***********************++::::::
 +++++++++*********++++++++******:::::::::
 ++++++++********++++++++++++**:::::::::::
 ++++++++*******++++++++++::::::::::::::::
 ++++++++******+++++++::::::::::::::::::::
 ++++++++*******++++::::::::::::::::::::::
 ++++++++********::::::::::**:::::::::::::
 +++++++++*********::::::******:::::::::::
 ++++++++::*********************::::::::::
 ++++++::::::*****************::::::::::::
   +:::::::::::*************::::::::::::
     :::::::::::::::::::::::::::::::::: 
       :::::::::::::::::::::::::::::
          :::::::::::::::::::::::
             :::::::::::::::::
 ";
 
 let mut colored_string = String::new();
 
 for c in logo.chars() {
   match c {
       '+' => colored_string.push_str(&c.to_string().truecolor(100, 149, 237).to_string()),
       '*' => colored_string.push_str(&c.to_string().white().to_string()),
       ':' => colored_string.push_str(&c.to_string().truecolor(0, 0, 139).to_string()),
       _ => colored_string.push(c),
   }
 }
 println!("{}", colored_string);} ,

          Language::CSharp => {
            let logo = 
 "
             +++++++++++++++++
           +++++++++++++++++++++++
       ++++++++++++++++++++++++++++
     +++++++++++++++++++++++++++++++++
   ++++++++++++*************++++++++++++
 ++++++++++++*******************++++++€€€€
 ++++++++++***********************++€€€€€€
 +++++++++*********++++++++******€€€€€€€€€
 ++++++++********++++++++++++**€€*€€*€€€€€
 ++++++++*******++++++++++€€€€€€******€€€€
 ++++++++******+++++++:::€€€€€€€€*€€*€€€€€
 ++++++++*******++++::::::::€€€€******€€€€
 ++++++++********::::::::::**€€€€*€€*€€€€€
 +++++++++*********::::::******€€€€€€€€€€€
 ++++++++::*********************€€€€€€€€€€
 ++++++::::::*****************:::€€€€€€€€€
   +:::::::::::*************:::::::€€€€€
     ::::::::::::::::::::::::::::::::€€ 
       :::::::::::::::::::::::::::::
          :::::::::::::::::::::::
             :::::::::::::::::
 ";
 
 let mut colored_string = String::new();
 
 for c in logo.chars() {
   match c {
       '+' => colored_string.push_str(&c.to_string().truecolor(178, 102, 255).to_string()),
       '*' => colored_string.push_str(&c.to_string().white().to_string()),
       ':' => colored_string.push_str(&c.to_string().truecolor(76, 0, 153).to_string()),
       '€' => colored_string.push_str(&c.to_string().truecolor(102, 0, 205).to_string()),
       _ => colored_string.push(c),
   }
 }
 println!("{}", colored_string);} ,

          Language::CPlusPlus =>  
          {
           let logo = 
"
            +++++++++++++++++
          +++++++++++++++++++++++
      ++++++++++++++++++++++++++++
    +++++++++++++++++++++++++++++++++
  ++++++++++++*************++++++++++++
++++++++++++*******************++++++::::
++++++++++***********************++::::::
+++++++++*********++++++++******:::::::::
++++++++********++++++++++++**:::::::::::
++++++++*******++++++++++::::::**:::**:::
++++++++******+++++++:::::::::****:****::
++++++++*******++++::::::::::::**:::**:::
++++++++********::::::::::**:::::::::::::
+++++++++*********::::::******:::::::::::
++++++++::*********************::::::::::
++++++::::::*****************::::::::::::
  +:::::::::::*************::::::::::::
    :::::::::::::::::::::::::::::::::: 
      :::::::::::::::::::::::::::::
         :::::::::::::::::::::::
            :::::::::::::::::
";

let mut colored_string = String::new();

for c in logo.chars() {
  match c {
      '+' => colored_string.push_str(&c.to_string().truecolor(100, 149, 237).to_string()),
      '*' => colored_string.push_str(&c.to_string().white().to_string()),
      ':' => colored_string.push_str(&c.to_string().truecolor(0, 0, 139).to_string()),
      _ => colored_string.push(c),
  }
}


println!("{}", colored_string);} ,
          Language::Java => todo!(),
          Language::Javascript => todo!(),
          Language::Typescript => todo!(),
          Language::Go => todo!(),
          Language::Rust => todo!(),
          Language::Scala => todo!(),
          Language::Haskell => todo!(),
          Language::Cobol => todo!(),
          Language::Ruby => todo!(),
  }
  
}

fn print_repository_data() {
  println!("Project:\n");
  println!("Head:\n");
  println!("Version:\n");
  println!("Created:\n");
  println!("Languages:\n");
  println!("Dependencies:\n");
  println!("Authors:\n");
  println!("Last Changes:\n");
  println!("Contributors:\n");
  println!("Repo:\n");
  println!("Commits:\n");
  println!("Lines of Code:\n");
  println!("Size:\n");
  println!("License:\n");
}

fn main() {
  print_language_logo(Language::CSharp);
  print_repository_data();
}
