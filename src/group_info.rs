use serde::{Deserialize, Serialize};
use crate::device::Capability;

/// Connectivity status of a device group
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GroupState {
    /// All devices in the group are online
    Online,
    /// All devices in the group are offline
    Offline,
    /// Some devices are online and some are offline
    Split,
}

/// Response containing detailed information about a device group
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupInfoResponse {
    /// Processing status (e.g., "ok")
    pub status: String,
    /// Unique request identifier for logging
    pub request_id: String,
    /// Group ID
    pub id: String,
    /// Group name
    pub name: String,
    /// List of additional names for the group
    pub aliases: Vec<String>,
    /// Group type (e.g., `devices.types.light`)
    #[serde(rename = "type")]
    pub group_type: String,
    /// Connectivity status of the group
    pub state: GroupState,
    /// List of capabilities for the group
    pub capabilities: Vec<Capability>,
    /// List of devices in this group
    pub devices: Vec<GroupDevice>,
}

/// Minimal device information within a group info response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupDevice {
    /// Device ID
    pub id: String,
    /// Device name
    pub name: String,
    /// Device type
    #[serde(rename = "type")]
    pub device_type: String,
}
