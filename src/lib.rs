use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::result::Result;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;
    //rather than panic! on an error, the ? allows the error value to be returned by the function
    
    println!("With text:\n{}", &contents);

    Ok(()) // if nothing goes wrong, just return ()
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(), //because this is a reference, we need to make a clone of it in the memory so it isnt dropped after the scope
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

}