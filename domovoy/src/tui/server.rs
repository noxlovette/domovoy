use crate::{
    Error, Res,
    auth::{NAME, SERVICE},
    tui::dispatcher::ColorAction,
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
        let info = self.0.user_info().await.map_err(|e| {
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

    pub async fn set_color(&self, device_id: &str, action: ColorAction) -> Res<()> {
        let (instance, value) = match action {
            ColorAction::Rgb(rgb) => ("rgb".to_string(), json!(rgb)),
            ColorAction::Hsv { h, s, v } => ("hsv".to_string(), json!({"h": h, "s": s, "v": v})),
            ColorAction::Temperature(k) => ("temperature_k".to_string(), json!(k)),
        };
        debug!(device_id, %instance, "sending colour action");
        let request = DeviceActionRequest {
            devices: vec![DeviceAction {
                id: device_id.to_string(),
                actions: vec![CapabilityAction {
                    capability_type: CapabilityType::ColorSetting,
                    state: CapabilityActionState { instance, value },
                }],
            }],
        };
        let result = self.0.device_actions(&request).await.map_err(|e| {
            error!(error = %e, "device_actions failed");
            Error::Api(e.to_string())
        })?;
        debug!(request_id = %result.request_id, status = ?result.status, "colour action response");
        Ok(())
    }
}
