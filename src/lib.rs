use std::{ error::Error, fs, env};

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    //   if args.len() < 4 {
    //       return Err("it need three arguments like \n cargo run query filename false");
    //   }
    //   let query = args[1].clone();
    //   let filename = args[2].clone();
    args.next();
    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string")
    };
    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a filename string")
    };
    let case_sensitive = match args.next() {
        Some(arg) => arg.to_string() == "true",
        None => return Err("Didn't get a case_sensitive string")
    };

    println!("=====>{}", case_sensitive);
    Ok(
        Config {
            query,
            filename,
            case_sensitive
        }
    )
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  // println!("With text: \n {}", contents);
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "Rust duct:\nsafe, fast, productive.\nPick three.\nDuct tape.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }
  
  #[test]
  fn case_insensitive() {
    let query = "rUSt";
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}
