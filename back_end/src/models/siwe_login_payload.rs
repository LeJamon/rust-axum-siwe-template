use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiweLoginPayload {
    pub message: String,
    pub signature: String,
}
