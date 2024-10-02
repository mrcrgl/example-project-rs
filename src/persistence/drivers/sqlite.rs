use crate::models::Zoo;
use crate::persistence::traits::PersistentModelWriter;

pub struct SqlitePersistenceDriver {
    config: SqlitePersistenceDriverConfig,
}

impl SqlitePersistenceDriver {
    pub fn new_with_config(config: SqlitePersistenceDriverConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl PersistentModelWriter for SqlitePersistenceDriver {
    async fn insert(&self, item: &Zoo) -> Result<(), crate::persistence::error::Error> {
        let item_data = serde_json::to_string_pretty(item).unwrap();
        println!("Saved item (sqlite: {}): \n{}", self.config.file, item_data,);
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct SqlitePersistenceDriverConfig {
    file: String,
}
