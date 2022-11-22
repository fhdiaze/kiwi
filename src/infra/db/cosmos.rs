use mongodb::{self, Client, options::ClientOptions};

use crate::{infra::errors::error::Result, domain::race::Race};

pub struct CosmosClient {
    client: Client,
    pub races: Collection<Race>
}

pub struct Collection<E> {
    collection: mongodb::Collection<E>
}

impl<E> Collection<E> {
    pub fn add(&self, e: E) -> String {
        self.collection.
    }
}

impl CosmosClient {
    pub async fn new(db_name: &str, connection_string: &str) -> Result<Self> {
        let options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(options)?;

        Ok(Self { client })
    }

    pub fn create<T>(&self, e: T) -> String {
        let doc = 
        self.client.database("abc").collection("abc").insert_one(doc, options)
    }
}
