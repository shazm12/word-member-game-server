use serde::{Deserialize, Serialize};

use super::word::Word;

#[derive(Serialize, Deserialize)]
pub struct WordArray {
    pub data: Vec<Word>
}
