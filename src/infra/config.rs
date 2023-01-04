pub struct Config {
    pub db: DbConfig,
}

pub struct DbConfig {
    pub connection_string: String,
    pub db_name: String,
    pub races_collection: String,
}

impl Config {
    pub fn new() -> Self {
        let db_config = DbConfig {
            connection_string: String::from("mongodb+srv://fredyd:beto8712@cluster0.ilsnrak.mongodb.net/?retryWrites=true&w=majority"),
            db_name: String::from("kiwi"),
            races_collection: String::from("races"),
        };

        Config { db: db_config }
    }
}
