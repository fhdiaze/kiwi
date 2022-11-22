pub struct Config {
    pub db: DbConfig,
}

pub struct DbConfig {
    pub connection_string: String,
    pub db_name: String,
}

impl Config {
    pub fn new() -> Self {
        let db_config = DbConfig {
            connection_string: String::from(""),
            db_name: String::from(""),
        };

        Config { db: db_config }
    }
}
