use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request to perform actions on multiple devices
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceActionRequest {
    /// List of devices and their target actions
    pub devices: Vec<DeviceAction>,
}

/// Actions for a specific device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceAction {
    /// Device ID
    pub id: String,
    /// List of actions to perform
    pub actions: Vec<CapabilityAction>,
}

/// Request to perform actions on all devices in a group
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupActionRequest {
    /// List of actions to perform
    pub actions: Vec<CapabilityAction>,
}

/// Representation of a target state for a device capability
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityAction {
    /// Capability type (e.g., "devices.capabilities.on_off")
    #[serde(rename = "type")]
    pub action_type: String,
    /// Target state
    pub state: CapabilityActionState,
}

/// The specific state instance and value for an action
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityActionState {
    /// Capability instance (e.g., "on")
    pub instance: String,
    /// New value for the instance
    pub value: Value,
}

/// Shared response for action requests
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionResponse {
    /// Processing status (e.g., "ok")
    pub status: String,
    /// Unique request identifier for logging
    pub request_id: String,
    /// Individual results for each affected device
    pub devices: Vec<DeviceActionResult>,
}

/// Result of actions performed on a specific device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceActionResult {
    /// Device ID
    pub id: String,
    /// List of capability action results
    pub capabilities: Vec<CapabilityActionResult>,
}

/// Result of an action on a specific capability
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityActionResult {
    /// Capability type
    #[serde(rename = "type")]
    pub action_type: String,
    /// The result of the state change
    pub state: CapabilityActionStateResult,
}

/// The outcome of an action on a capability instance
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityActionStateResult {
    /// Capability instance
    pub instance: String,
    /// Result of the action (status and potential errors)
    pub action_result: ActionResultStatus,
}

/// Status of a specific action
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionResultStatus {
    /// Status: "DONE" or "ERROR"
    pub status: String,
    /// Error code (if status is "ERROR")
    pub error_code: Option<String>,
    /// Human-readable error message (if status is "ERROR")
    pub error_message: Option<String>,
}
