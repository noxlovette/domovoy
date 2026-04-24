#![warn(missing_docs)]
//! The domovoy library
use reqwest::Method;

mod device;
/// The host for all requests
pub const HOST: &str = "https://api.iot.yandex.net";

/// [Device control protocol](https://yandex.ru/dev/dialogs/smart-home/doc/en/concepts/platform-protocol)
pub enum Endpoint {
    /// Gets full info about the user's smart home setup
    UserInfo,
    /// Gets info about device status
    DeviceStatus {
        /// The id of the device referred to
        device_id: String,
    },
    /// Controls device actions
    DeviceActions,
    /// Gets group status
    GroupStatus {
        /// The id of the group referred to
        group_id: String,
    },
    /// Controls group actions
    GroupActions {
        /// The id of the group referred to
        group_id: String,
    },
    /// Controls scenarios
    ScenarioActions {
        /// The id of the scenario referred to
        scenario_id: String,
    },
    /// Deletes a device
    DeviceDelete {
        /// The id of the device referred to
        device_id: String,
    },
}

impl Endpoint {
    /// Returns the method for a specific endpoint
    pub fn method(&self) -> Method {
        use Endpoint::*;

        match self {
            UserInfo | DeviceStatus { .. } | GroupStatus { .. } => Method::GET,
            DeviceActions { .. } | GroupActions { .. } | ScenarioActions { .. } => Method::POST,
            _ => Method::DELETE,
        }
    }
    /// Returns the path for a specific endpoint
    pub fn path(&self) -> String {
        match self {
            Self::UserInfo => "v1.0/user/info".to_string(),
            Self::DeviceStatus { device_id } => format!("v1.0/devices/{device_id}"),
            Self::DeviceActions => "v1.0/devices/actions".to_string(),
            Self::GroupStatus { group_id } => format!("v1.0/groups/{group_id}"),
            Self::GroupActions { group_id } => format!("v1.0/groups/{group_id}/actions"),
            Self::DeviceDelete { device_id } => format!("v1.0/devices/{device_id}"),
            Self::ScenarioActions { scenario_id } => {
                format!("v1.0/scenarios/{scenario_id}/actions")
            }
        }
    }
}
