use serde::{Deserialize, Serialize};
use crate::device::{Capability, Property};

/// Connectivity status of a device
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeviceState {
    /// Device is online and reachable
    Online,
    /// Device is offline
    Offline,
}

/// Response containing detailed information about a single device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfoResponse {
    /// Processing status (e.g., "ok")
    pub status: String,
    /// Unique request identifier for logging
    pub request_id: String,
    /// Unique device ID
    pub id: String,
    /// User-defined name of the device
    pub name: String,
    /// List of additional names for the device
    pub aliases: Vec<String>,
    /// Device type (e.g., `devices.types.light`)
    #[serde(rename = "type")]
    pub device_type: String,
    /// Connectivity status: "online" or "offline"
    pub state: DeviceState,
    /// List of group IDs the device belongs to
    pub groups: Vec<String>,
    /// Room ID (can be null if not assigned)
    pub room: Option<String>,
    /// ID in the manufacturer's cloud
    pub external_id: String,
    /// ID of the manufacturer's skill
    pub skill_id: String,
    /// List of device capabilities
    pub capabilities: Vec<Capability>,
    /// List of device properties
    pub properties: Vec<Property>,
}
