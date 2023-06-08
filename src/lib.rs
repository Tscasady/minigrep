use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
         
        let query = args[1].to_owned();
        let file_path = args[2].to_owned();

        Ok(Config { query, file_path })
    }
}

    //The Box... is a trait object. This function will return a type that implements the Error trait,
//but we don't specify what that type is.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("\nText: \n{contents}");

    Ok(())
}
