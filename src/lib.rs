mod schema_1;
mod schema_2;

pub use schema_1::Schema1;
pub use schema_2::{Schema2, Schema2Id};

use mongodb::{Client, Collection, Database};
use once_cell::sync::OnceCell;
use serde::Deserializer;

use mongo_schema_macro::MongoSchema;

pub static MONGO_DATABASE: OnceCell<Database> = OnceCell::new();
pub static DATABASE_NAME: &str = "test_db";

pub async fn init_client(uri: &str) {
    let client = Client::with_uri_str(uri).await.unwrap();
    let database = client.database(DATABASE_NAME);
    MONGO_DATABASE.set(database).unwrap();
}

pub fn collection_deserializer<'de, D, T>(_: D) -> Result<Collection<T>, D::Error>
where
    D: Deserializer<'de>,
    T: MongoSchema,
{
    Ok(MONGO_DATABASE
        .get()
        .expect("Looks like the mongo database has not been initialized.")
        .collection::<T>(T::collection_name()))
}

// pub trait Named {
//     fn name() -> &'static str;
// }
