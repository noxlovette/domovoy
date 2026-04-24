#![warn(missing_docs)]
//! The domovoy library
use reqwest::{Method, Url};

use crate::error::Res;
mod actions;
mod device;
mod device_info;
mod error;
mod group_info;
mod simple_response;
mod user_info;

pub use actions::*;
pub use device::*;
pub use device_info::*;
pub use group_info::*;
pub use simple_response::*;
pub use user_info::*;
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
    /// Returns the full url for a specific endpoint
    pub fn url(&self) -> Res<Url> {
        let mut base = Url::parse(HOST)?;
        let path = match self {
            Self::UserInfo => "v1.0/user/info".to_string(),
            Self::DeviceStatus { device_id } => format!("v1.0/devices/{device_id}"),
            Self::DeviceActions => "v1.0/devices/actions".to_string(),
            Self::GroupStatus { group_id } => format!("v1.0/groups/{group_id}"),
            Self::GroupActions { group_id } => format!("v1.0/groups/{group_id}/actions"),
            Self::DeviceDelete { device_id } => format!("v1.0/devices/{device_id}"),
            Self::ScenarioActions { scenario_id } => {
                format!("v1.0/scenarios/{scenario_id}/actions")
            }
        };
        base.set_path(&path);

        Ok(base)
    }
}
