use serde::{Deserialize, Serialize};
use crate::device::Device;

/// Request for full information about the user's smart home
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserInfoRequest {}

/// Response containing full information about the user's smart home
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoResponse {
    /// Processing status (e.g., "ok")
    pub status: String,
    /// Unique request identifier for logging
    pub request_id: String,
    /// List of rooms
    pub rooms: Vec<Room>,
    /// List of groups
    pub groups: Vec<Group>,
    /// List of devices
    pub devices: Vec<Device>,
    /// List of scenarios
    pub scenarios: Vec<Scenario>,
    /// List of households
    pub households: Vec<Household>,
}

/// Represents a room in the smart home
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    /// Room ID
    pub id: String,
    /// Room name
    pub name: String,
    /// ID of the household the room belongs to
    pub household_id: String,
    /// List of device IDs in this room
    pub devices: Vec<String>,
}

/// Represents a group of devices
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    /// Group ID
    pub id: String,
    /// Group name
    pub name: String,
    /// List of additional names for the group
    pub aliases: Vec<String>,
    /// Group type (e.g., `devices.types.light`)
    #[serde(rename = "type")]
    pub group_type: String,
    /// ID of the household the group belongs to
    pub household_id: String,
    /// List of device IDs in this group
    pub devices: Vec<String>,
    /// List of group capabilities
    pub capabilities: Vec<serde_json::Value>,
}

/// Represents a scenario
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scenario {
    /// Scenario ID
    pub id: String,
    /// Scenario name
    pub name: String,
    /// Whether the scenario is active
    pub is_active: bool,
}

/// Represents a household
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Household {
    /// Household ID
    pub id: String,
    /// Household name
    pub name: String,
}
