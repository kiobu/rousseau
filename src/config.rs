pub struct Config {
    token: String,
    prefix: String
}

impl Config {
    pub fn create() -> Config {
        Config {
            token: String::from(""),
            prefix: String::from("--")
        }
    }
    pub fn prefix() -> String {
        Config::create().prefix
    }
    pub fn token() -> String {
        Config::create().token
    }
}