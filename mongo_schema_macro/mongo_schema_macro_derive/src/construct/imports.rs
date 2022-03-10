use quote::quote;

pub fn imports() -> proc_macro2::TokenStream {
    quote! {
        use async_trait::async_trait;
        use mongodb::{
            bson::{
                doc,
                oid::ObjectId
            },
            Collection
        };
        use serde::{Serialize, Deserialize};

        use mongo_schema::{
            collection_deserializer,
            MONGO_DATABASE
        };
    }
}