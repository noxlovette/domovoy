use serde::{Deserialize, Serialize};

/// Simple response containing only status and request ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleResponse {
    /// Processing status (e.g., "ok")
    pub status: String,
    /// Unique request identifier for logging
    pub request_id: String,
}
