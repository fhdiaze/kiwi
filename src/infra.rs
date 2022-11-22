pub mod db;
pub mod config;
pub mod errors;

pub struct Infra {
    mongo_client: Option<db::mongo::Client>
}

static INSTANCE: Infra = Infra {
    mongo_client: Some(db::mongo::Client::new(config::Config::INSTANCE.db_config)
};