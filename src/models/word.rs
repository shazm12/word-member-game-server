use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Word {
    pub name: String,
    pub members: Vec<String>,
    pub random : Option<i32>
}
