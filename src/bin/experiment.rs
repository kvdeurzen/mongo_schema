use mongo_schema::{Schema1, Schema2};
use mongo_schema_macro::MongoSchema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = Client::with_uri_str("mongodb://localhost:27017")
    //     .await
    //     .unwrap();

    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    mongo_schema::init_client("mongodb://localhost:27017").await;

    let mut s1 = Schema1::new();
    let mut s2 = Schema2::new();
    println!("{:?} - {:?}", s1, s2);

    s1.save().await?;
    s2.save().await?;
    println!("{:?} - {:?}", s1, s2);

    s1.connected_schema2 = Some(s2.id.unwrap().into());
    s1.save().await?;

    Ok(())
}
