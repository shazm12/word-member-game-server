use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Success {
    pub message: String,
}
