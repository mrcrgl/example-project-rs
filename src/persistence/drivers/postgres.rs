use crate::models::Zoo;
use crate::persistence::traits::PersistentModelWriter;

pub struct PostgresPersistenceDriver {
    config: PostgresPersistenceDriverConfig,
}

impl PostgresPersistenceDriver {
    pub fn new_with_config(config: PostgresPersistenceDriverConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl PersistentModelWriter for PostgresPersistenceDriver {
    async fn insert(&self, item: &Zoo) -> Result<(), crate::persistence::error::Error> {
        println!(
            "Saved item (postgres: {}/{}): \n{}",
            self.config.host,
            self.config.table,
            serde_json::to_string_pretty(item).unwrap()
        );
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PostgresPersistenceDriverConfig {
    host: String,
    table: String,
}
