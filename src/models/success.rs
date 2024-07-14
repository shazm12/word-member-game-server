use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Success {
    message: String,
}
