use mongodb;

use crate::{infra::{errors::error::Result, config::DbConfig}, domain::race::Race};

pub struct Client {
    client: mongo::Client,
    pub races: Collection<Race>
}

impl Client {
    pub async fn new(config: &DbConfig) -> Result<Self> {
        let options = mongodb::ClientOptions::parse(connection_string).await?;
        let client = mongodb::Client::with_options(options)?;
        let db = client.database(&config.db_name);
        let races = db.collection("");

        Ok(Self { client, races })
    }

    pub fn create<T>(&self, e: T) -> String {
        let doc = 
        self.client.database("abc").collection("abc").insert_one(doc, options)
    }
}

pub struct Collection<E> {
    collection: mongodb::Collection<E>
}

impl<E> Collection<E> {
    pub fn add(&self, e: E) -> String {
        self.collection.
    }
}