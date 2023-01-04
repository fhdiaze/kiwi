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
            connection_string: String::from("mongodb://localhost:C2y6yDjf5%2FR%2Bob0N8A7Cgv30VRDJIWEHLM%2B4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw%2FJw%3D%3D@localhost:10255/admin?ssl=true"),
            db_name: String::from("kiwi"),
            races_collection: String::from("races"),
        };

        Config { db: db_config }
    }
}
