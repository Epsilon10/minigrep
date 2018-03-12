use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::result::Result;
use std::env;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;
    //rather than panic! on an error, the ? allows the error value to be returned by the function

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    println!("With text:\n{}", &contents);

    Ok(()) // if nothing goes wrong, just return ()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(), //because this is a reference, we need to make a clone of it in the memory so it isnt dropped after the scope
            case_sensitive,
        })
    }
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
/*
    In other words, we tell Rust that the data returned by the search function will 
    live as long as the data passed into the search function in the contents argument. 
*/

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); //query is now a String not a string slice (&str) so now we need to use & to make sure it does not lose ownershuip
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}