use config::PersistenceDriver;
use drivers::{postgres::PostgresPersistenceDriver, sqlite::SqlitePersistenceDriver};

pub(crate) mod config;
pub(crate) mod drivers;
pub(crate) mod error;
pub(crate) mod traits;

pub fn load_driver(driver: PersistenceDriver) -> Box<dyn traits::PersistentModelWriter> {
    match driver {
        PersistenceDriver::Sqlite(config) => {
            Box::new(SqlitePersistenceDriver::new_with_config(config))
        }
        PersistenceDriver::Postgres(config) => {
            Box::new(PostgresPersistenceDriver::new_with_config(config))
        }
    }
}
