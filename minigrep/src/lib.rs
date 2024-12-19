use std::env;
use std::fs;
use std::error::Error;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename())?;
    let results = if config.ignore_case() {
        search_case_insensitive(config.query(), &contents)
    } else {
        search(config.query(), &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }
    matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }
    matches
}

pub fn fatal(error: &str) -> ! {
    eprintln!("Error: {}", error);
    process::exit(1);
}

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool
}

impl Config {
    fn new(query: &String, filename: &String, ignore_case: bool) -> Self {
        Self { query: query.clone(), filename: filename.clone(), ignore_case }
    }

    pub fn from_args() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self::new(&args[1], &args[2], ignore_case))
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
