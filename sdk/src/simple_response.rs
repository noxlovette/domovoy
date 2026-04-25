use serde::{Deserialize, Serialize};

/// Processing status returned by every API response
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    /// Request processed successfully
    Ok,
    /// Request failed
    Error,
}

/// Minimal response containing only status and request ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleResponse {
    /// Processing status
    pub status: ResponseStatus,
    /// Unique request identifier for incident investigation
    pub request_id: String,
}
