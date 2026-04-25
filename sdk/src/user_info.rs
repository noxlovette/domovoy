use serde::{Deserialize, Serialize};

use crate::device::{Device, DeviceType, GroupCapability};
use crate::simple_response::ResponseStatus;

/// Full information about the user's smart home
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoResponse {
    /// Processing status
    pub status: ResponseStatus,
    /// Unique request identifier for incident investigation
    pub request_id: String,
    /// Rooms in the household
    pub rooms: Vec<Room>,
    /// Device groups
    pub groups: Vec<Group>,
    /// All devices
    pub devices: Vec<Device>,
    /// User-defined scenarios
    pub scenarios: Vec<Scenario>,
    /// Households associated with the account
    pub households: Vec<Household>,
}

/// A room in the smart home
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    /// Room ID
    pub id: String,
    /// Room name
    pub name: String,
    /// ID of the household this room belongs to
    pub household_id: String,
    /// IDs of devices assigned to this room
    pub devices: Vec<String>,
}

/// A group of devices
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    /// Group ID
    pub id: String,
    /// Group name
    pub name: String,
    /// Additional user-defined names
    pub aliases: Vec<String>,
    /// Group type (mirrors the device type of its members)
    #[serde(rename = "type")]
    pub group_type: DeviceType,
    /// ID of the household this group belongs to
    pub household_id: String,
    /// IDs of devices in this group
    pub devices: Vec<String>,
    /// Shared capabilities of the group
    pub capabilities: Vec<GroupCapability>,
}

/// A user-defined scenario
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scenario {
    /// Scenario ID
    pub id: String,
    /// Scenario name
    pub name: String,
    /// Whether the scenario is currently enabled
    pub is_active: bool,
}

/// A household associated with the account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Household {
    /// Household ID
    pub id: String,
    /// Household name
    pub name: String,
}
