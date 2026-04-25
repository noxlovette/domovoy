use serde::{Deserialize, Serialize};

use crate::device::{DeviceType, GroupCapability};
use crate::simple_response::ResponseStatus;

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

/// Detailed information about a device group
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupInfoResponse {
    /// Processing status
    pub status: ResponseStatus,
    /// Unique request identifier for incident investigation
    pub request_id: String,
    /// Group ID
    pub id: String,
    /// Group name
    pub name: String,
    /// Additional user-defined names
    pub aliases: Vec<String>,
    /// Group type (mirrors the device type of its members)
    #[serde(rename = "type")]
    pub group_type: DeviceType,
    /// Connectivity status of the group
    pub state: GroupState,
    /// Shared capabilities of the group
    pub capabilities: Vec<GroupCapability>,
    /// Devices that belong to this group
    pub devices: Vec<GroupDevice>,
}

/// Minimal device entry within a group info response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupDevice {
    /// Device ID
    pub id: String,
    /// Device name
    pub name: String,
    /// Device type
    #[serde(rename = "type")]
    pub device_type: DeviceType,
}
