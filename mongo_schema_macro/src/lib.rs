use async_trait::async_trait;

#[async_trait]
pub trait MongoSchema {
    fn collection_name() -> &'static str;
    async fn post(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn update(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn save(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
