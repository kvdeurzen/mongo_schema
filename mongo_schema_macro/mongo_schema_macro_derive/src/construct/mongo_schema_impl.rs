use quote::quote;

use crate::{
    SchemaAttribute,
    SchemaDefinition,
};

pub fn mongo_schema_impl(def: &SchemaDefinition, attr: &SchemaAttribute) -> proc_macro2::TokenStream {
    let struct_name  = &def.ident;
    let collection_name = (attr.collection).clone().unwrap();

    quote! {
        #[async_trait]
        impl MongoSchema for #struct_name {

            fn collection_name() -> &'static str {
                #collection_name
            }

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
    }
}