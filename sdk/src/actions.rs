use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::device::CapabilityType;
use crate::simple_response::ResponseStatus;

/// Request to perform actions on multiple devices
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceActionRequest {
    /// Devices and the actions to apply to each
    pub devices: Vec<DeviceAction>,
}

/// Actions targeting a specific device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceAction {
    /// Device ID
    pub id: String,
    /// Actions to perform on this device
    pub actions: Vec<CapabilityAction>,
}

/// Request to perform actions on all devices in a group
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupActionRequest {
    /// Actions to apply to every device in the group
    pub actions: Vec<CapabilityAction>,
}

/// Target state for a single capability
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityAction {
    /// Capability type
    #[serde(rename = "type")]
    pub capability_type: CapabilityType,
    /// Desired state
    pub state: CapabilityActionState,
}

/// Instance and value that define the desired capability state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityActionState {
    /// Capability instance (e.g. `"on"`, `"brightness"`)
    pub instance: String,
    /// New value for the instance
    pub value: Value,
}

/// Outcome of a single capability action
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ActionStatus {
    /// State was changed successfully
    #[serde(rename = "DONE")]
    Done,
    /// State change failed
    #[serde(rename = "ERROR")]
    Error,
}

/// Response for device or group action requests
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionResponse {
    /// Processing status
    pub status: ResponseStatus,
    /// Unique request identifier for incident investigation
    pub request_id: String,
    /// Per-device action results
    pub devices: Vec<DeviceActionResult>,
}

/// Action results for a specific device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceActionResult {
    /// Device ID
    pub id: String,
    /// Per-capability action results
    pub capabilities: Vec<CapabilityActionResult>,
}

/// Result of an action on a specific capability
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityActionResult {
    /// Capability type
    #[serde(rename = "type")]
    pub capability_type: CapabilityType,
    /// Outcome of the state change
    pub state: CapabilityActionStateResult,
}

/// Outcome of an action on a capability instance
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CapabilityActionStateResult {
    /// Capability instance
    pub instance: String,
    /// Result of the action
    pub action_result: ActionResult,
}

/// Result of a capability state-change attempt
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionResult {
    /// Whether the action succeeded
    pub status: ActionStatus,
    /// Machine-readable error code (present when `status` is `Error`)
    pub error_code: Option<String>,
    /// Human-readable error description (present when `status` is `Error`)
    pub error_message: Option<String>,
}
