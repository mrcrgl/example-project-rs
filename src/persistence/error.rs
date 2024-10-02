#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an IO error occurred")]
    IO,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
