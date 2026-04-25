use crate::{
    Error, Res,
    auth::{NAME, SERVICE},
};
use keyring::Entry;
use serde_json::json;
use tracing::{debug, error};
use yandex_home_sdk::{
    CapabilityAction, CapabilityActionState, CapabilityType, Client, Device, DeviceAction,
    DeviceActionRequest, DeviceType,
};

pub struct Server(Client);

impl Server {
    pub fn new() -> Res<Self> {
        let entry = Entry::new(SERVICE, NAME).map_err(|_| Error::LoginNeeded)?;
        let token = entry.get_password().map_err(|_| Error::LoginNeeded)?;
        let client = Client::new(token).map_err(|_| Error::LoginNeeded)?;
        Ok(Self(client))
    }

    pub async fn light_devices(&self) -> Res<Vec<Device>> {
        debug!("fetching user info");
        let info = self
            .0
            .user_info()
            .await
            .map_err(|e| {
                error!(error = %e, "user_info failed");
                Error::Api(e.to_string())
            })?;
        debug!(total = info.devices.len(), "devices received");
        let lights: Vec<Device> = info
            .devices
            .into_iter()
            .filter(|d| {
                d.device_type == DeviceType::Light
                    && d.capabilities
                        .iter()
                        .any(|c| c.capability_type == CapabilityType::ColorSetting)
            })
            .collect();
        debug!(colour_lights = lights.len(), "filtered colour lights");
        Ok(lights)
    }

    pub async fn set_color(&self, device_id: &str, rgb: u32) -> Res<()> {
        debug!(device_id, rgb, "sending colour action");
        let request = DeviceActionRequest {
            devices: vec![DeviceAction {
                id: device_id.to_string(),
                actions: vec![CapabilityAction {
                    capability_type: CapabilityType::ColorSetting,
                    state: CapabilityActionState {
                        instance: "rgb".to_string(),
                        value: json!(rgb),
                    },
                }],
            }],
        };
        let result = self.0.device_actions(&request).await.map_err(|e| {
            error!(error = %e, "device_actions failed");
            Error::Api(e.to_string())
        })?;
        debug!(request_id = %result.request_id, status = ?result.status, "colour action response");
        for dev in &result.devices {
            for cap in &dev.capabilities {
                debug!(
                    device = %dev.id,
                    instance = %cap.state.instance,
                    status = ?cap.state.action_result.status,
                    error_code = ?cap.state.action_result.error_code,
                    "capability result"
                );
            }
        }
        Ok(())
    }
}
