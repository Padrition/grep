use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensetive: bool,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("No query was specified!"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("No file name was specified!"),  
        };

        let case_sensetive = env::var("CASE_SENSETIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensetive,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensetive {
        search(&config.query, &contents)
    } else {
        search_case_insensetive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensetive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensetive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensetive(query, contents)
        );
    }
}
