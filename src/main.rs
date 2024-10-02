use crate::models::Zoo;
use config::Config;
use models::Animal;

pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod models;
pub(crate) mod persistence;

#[tokio::main]
async fn main() {
    let config = match Config::try_from_file("config.yaml").await {
        Ok(config) => config,
        Err(_) => {
            panic!("failed to load config: config.yaml");
        }
    };

    let zoo = Zoo {
        animals: vec![Animal {
            name: "lion".to_string(),
            can_growl: true,
        }],
    };

    let db = persistence::load_driver(config.persistence_driver);

    match db.insert(&zoo).await {
        Ok(_) => {
            println!("inserted successfully!");
        }
        Err(err) => {
            println!("inserted an error: {err}");
        }
    };
}
