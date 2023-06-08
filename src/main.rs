use std::env;
use std::fs;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("\nSearching for {} in file {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("\nText: \n{contents}");
}
