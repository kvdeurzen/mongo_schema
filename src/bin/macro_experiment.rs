use mongo_schema_macro::MongoSchema;
use mongo_schema_macro_derive::mongo_schema;

#[mongo_schema(collection = "schema_1")]
struct Schema1 {
    // #[relation(reference)]
    pub us_field: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    mongo_schema::init_client("mongodb://localhost:27017").await;

    let mut schema1 = Schema1::new(10);

    // println!("{:?}", schema1);

    let res = schema1.save().await;

    println!("{:?}", res);

    Ok(())
}
