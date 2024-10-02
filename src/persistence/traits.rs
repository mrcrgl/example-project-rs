use crate::models::Zoo;

#[async_trait::async_trait]
pub trait PersistentModelWriter {
    async fn insert(&self, item: &Zoo) -> Result<(), super::error::Error>;
}
