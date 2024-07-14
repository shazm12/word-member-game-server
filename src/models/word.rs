use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Word {
    name: String,
    members: Vec<String>,
}
