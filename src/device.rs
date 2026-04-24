use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a device in the Yandex Smart Home
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    /// Unique device ID
    pub id: String,
    /// User-defined name of the device
    pub name: String,
    /// List of additional names for the device
    pub aliases: Vec<String>,
    /// Device type (e.g., `devices.types.light`)
    #[serde(rename = "type")]
    pub device_type: String,
    /// ID in the manufacturer's cloud
    pub external_id: String,
    /// ID of the manufacturer's skill
    pub skill_id: String,
    /// ID of the household the device belongs to
    pub household_id: String,
    /// Room ID (can be null if not assigned)
    pub room: Option<String>,
    /// List of group IDs the device belongs to
    pub groups: Vec<String>,
    /// List of device capabilities
    pub capabilities: Vec<Capability>,
    /// List of device properties
    pub properties: Vec<Property>,
}

/// Describes what a device can do (e.g., turn on/off, change color)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Capability {
    /// Capability type
    #[serde(rename = "type")]
    pub capability_type: String,
    /// Whether the capability can be reported to the platform
    pub reportable: bool,
    /// Whether the capability state can be retrieved
    pub retrievable: bool,
    /// Parameters of the capability
    pub parameters: Value,
    /// Current state of the capability
    pub state: Option<Value>,
    /// Time of the last state update
    pub last_updated: f64,
}

/// Describes device sensors or read-only states (e.g., battery level, temperature)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    /// Property type
    #[serde(rename = "type")]
    pub property_type: String,
    /// Whether the property can be reported to the platform
    pub reportable: bool,
    /// Whether the property state can be retrieved
    pub retrievable: bool,
    /// Parameters of the property
    pub parameters: Value,
    /// Current state of the property
    pub state: Option<Value>,
    /// Time of the last state update
    pub last_updated: f64,
}
