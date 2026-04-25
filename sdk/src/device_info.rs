use serde::{Deserialize, Serialize};

use crate::device::{DeviceCapability, DeviceProperty, DeviceType};
use crate::simple_response::ResponseStatus;

/// Connectivity status of a device
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeviceState {
    /// Device is reachable
    Online,
    /// Device is not reachable
    Offline,
}

/// Detailed information about a single device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfoResponse {
    /// Processing status
    pub status: ResponseStatus,
    /// Unique request identifier for incident investigation
    pub request_id: String,
    /// Unique device ID
    pub id: String,
    /// User-defined name
    pub name: String,
    /// Additional user-defined names
    pub aliases: Vec<String>,
    /// Device type
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    /// Connectivity status
    pub state: DeviceState,
    /// IDs of groups this device belongs to
    pub groups: Vec<String>,
    /// Room ID (`None` if not assigned to a room)
    pub room: Option<String>,
    /// ID in the manufacturer's cloud
    pub external_id: String,
    /// ID of the manufacturer's skill
    pub skill_id: String,
    /// Device capabilities with current state
    pub capabilities: Vec<DeviceCapability>,
    /// Device properties with current state
    pub properties: Vec<DeviceProperty>,
}
