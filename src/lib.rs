use std::fs;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    if config.case_sensitive {
        for line in search(&config.query, &content) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{}", line);
        }
    }

    Ok(())
}


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(q) => q,
            None => return Err("Didn't get the query."),
        };

        let filename = match args.next() {
            Some(q) => q,
            None => return Err("Didn't get the filename."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}


pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use crate::search;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duck";
        let content = "\
                       hey hey duck you are hot.
                       I love you.";

        assert_eq!(vec!["hey hey duck you are hot."], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let content = "if you want the best, then you want rust.";
        assert_eq!(vec!["if you want the best, then you want rust."], search_case_insensitive(query, content));
    }
}
