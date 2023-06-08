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

