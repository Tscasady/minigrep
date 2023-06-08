pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args[1].to_owned();
        let file_path = args[2].to_owned();

        Config { query, file_path }
    }
}

