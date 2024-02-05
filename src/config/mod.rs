pub struct Config {
    pub postgres: String,
    pub bigchain: String,
}

impl Config {
    pub fn new(postgres: String, bigchain: String) -> Self {
        Self { postgres, bigchain }
    }
}
