use std::{fmt, fmt::Debug};

use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};

use mongo_schema_macro::MongoSchema;

use super::{collection_deserializer, MONGO_DATABASE};

#[derive(Serialize, Deserialize, Clone)]
pub struct Schema2Id {
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
}

impl From<ObjectId> for Schema2Id {
    fn from(object: ObjectId) -> Self {
        Self { id: Some(object) }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Schema2 {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing)]
    #[serde(deserialize_with = "collection_deserializer::<_, Schema2>")]
    collection: Collection<Self>,
}

impl Debug for Schema2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Schema1").field("id", &self.id).finish()
    }
}

impl Schema2 {
    pub fn new() -> Self {
        let id = None;
        let collection = MONGO_DATABASE
            .get()
            .unwrap()
            .collection::<Self>(Self::collection_name());
        Self { id, collection }
    }
}

#[async_trait]
impl MongoSchema for Schema2 {
    fn collection_name() -> &'static str {
        "Schema2"
    }

    // async fn from_id(id: &Schema2Id) -> Result<Option<Self>, Box<dyn std::error::Error>> {
    //     let collection = MONGO_DATABASE.get().unwrap().collection::<Self>(COLLECTION_NAME);
    //     collection.find_one( doc!( "id": id.id ), None).await.map_err(|e| e.into())
    // }

    async fn post(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let insert = self.clone();
        let res = self.collection.insert_one(insert, None).await?;
        self.id = Some(res.inserted_id.as_object_id().unwrap());
        Ok(())
    }

    async fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let query = doc! { "_id" : self.id };
        let update = doc! { "$set": bson::to_document(&self).unwrap() };
        let _ = self.collection.update_one(query, update, None).await?;
        Ok(())
    }

    async fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = self.id {
            self.update().await?;
        } else {
            self.post().await?;
        }
        Ok(())
    }
}
