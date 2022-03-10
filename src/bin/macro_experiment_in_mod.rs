pub mod test {
    use mongo_schema_macro::MongoSchema;
    use mongo_schema_macro_derive::mongo_schema;

    #[mongo_schema(collection = "schema_1")]
    pub struct Schema1 {
        // #[relation(reference)]
        pub us_field: usize,
    }
}

use mongo_schema_macro::MongoSchema;

fn main() {
    let schema1 = test::Schema1::new(10);

    println!("{:?}", schema1.us_field);
    schema1.hello();
}
