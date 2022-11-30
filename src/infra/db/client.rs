use mongodb;

use crate::{
    domain::race::Race,
    infra::{config::DbConfig, errors::error::Result},
};

pub struct Client {
    client: mongodb::Client,
    races: mongodb::Collection<Race>,
}

impl Client {
    pub async fn new(config: &DbConfig) -> Result<Self> {
        let options = mongodb::options::ClientOptions::parse(&config.connection_string).await?;
        let client = mongodb::Client::with_options(options)?;
        let db = client.database(&config.db_name);
        let races = db.collection::<Race>(&config.races_collection);

        Ok(Self { client, races })
    }
}

impl super::traits::Client for Client {
    fn races(&self) -> mongodb::Collection<Race> {
        self.races
    }
}

