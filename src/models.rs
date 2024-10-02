#[derive(serde::Serialize, serde::Deserialize)]
pub struct Zoo {
    pub animals: Vec<Animal>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Animal {
    pub name: String,
    pub can_growl: bool,
}
