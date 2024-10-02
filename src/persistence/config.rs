use super::drivers::{
    postgres::PostgresPersistenceDriverConfig, sqlite::SqlitePersistenceDriverConfig,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(tag = "driver", content = "config")]
pub enum PersistenceDriver {
    #[serde(rename = "sqlite")]
    Sqlite(SqlitePersistenceDriverConfig),
    #[serde(rename = "postgres")]
    Postgres(PostgresPersistenceDriverConfig),
}
