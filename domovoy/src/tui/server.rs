use crate::{
    Error, Res,
    auth::{NAME, SERVICE},
    tui::dispatcher::ColorAction,
};
use keyring::Entry;
use tracing::{debug, error};
use yandex_home_sdk::{
    CapabilityAction, Client, ColorSettingState, Device, DeviceAction, DeviceActionRequest,
    DeviceCapability, DeviceType, Group, GroupActionRequest, GroupCapability, HsvColor, OnOffState,
};

pub struct Server(Client);

impl Server {
    pub fn new() -> Res<Self> {
        let entry = Entry::new(SERVICE, NAME).map_err(|_| Error::LoginNeeded)?;
        let token = entry.get_password().map_err(|_| Error::LoginNeeded)?;
        let client = Client::new(token).map_err(|_| Error::LoginNeeded)?;
        Ok(Self(client))
    }

    pub async fn light_info(&self) -> Res<(Vec<Device>, Vec<Group>)> {
        debug!("fetching user info");
        let info = self.0.user_info().await.map_err(|e| {
            error!(error = %e, "user_info failed");
            Error::Api(e.to_string())
        })?;
        debug!(total_devices = info.devices.len(), total_groups = info.groups.len(), "info received");

        let devices = info
            .devices
            .into_iter()
            .filter(|d| {
                d.device_type == DeviceType::Light
                    && d.capabilities
                        .iter()
                        .any(|c| matches!(c, DeviceCapability::ColorSetting { .. }))
            })
            .collect();

        let groups = info
            .groups
            .into_iter()
            .filter(|g| {
                g.group_type == DeviceType::Light
                    && g.capabilities
                        .iter()
                        .any(|c| matches!(c, GroupCapability::ColorSetting { .. }))
            })
            .collect();

        Ok((devices, groups))
    }

    pub async fn set_color(&self, device_id: &str, action: ColorAction) -> Res<()> {
        let state = color_action_to_state(action);
        debug!(device_id, "sending colour action");
        let request = DeviceActionRequest {
            devices: vec![DeviceAction {
                id: device_id.to_string(),
                actions: vec![CapabilityAction::ColorSetting { state }],
            }],
        };
        let result = self.0.device_actions(&request).await.map_err(|e| {
            error!(error = %e, "device_actions failed");
            Error::Api(e.to_string())
        })?;
        debug!(request_id = %result.request_id, status = ?result.status, "colour action response");
        Ok(())
    }

    pub async fn set_group_color(&self, group_id: &str, action: ColorAction) -> Res<()> {
        let state = color_action_to_state(action);
        debug!(group_id, "sending group colour action");
        let request = GroupActionRequest {
            actions: vec![CapabilityAction::ColorSetting { state }],
        };
        let result = self.0.group_actions(group_id, &request).await.map_err(|e| {
            error!(error = %e, "group_actions failed");
            Error::Api(e.to_string())
        })?;
        debug!(request_id = %result.request_id, "group colour response");
        Ok(())
    }

    pub async fn toggle_device(&self, device_id: &str, on: bool) -> Res<()> {
        debug!(device_id, on, "sending on/off action");
        let request = DeviceActionRequest {
            devices: vec![DeviceAction {
                id: device_id.to_string(),
                actions: vec![CapabilityAction::OnOff {
                    state: OnOffState::On(on),
                }],
            }],
        };
        let result = self.0.device_actions(&request).await.map_err(|e| {
            error!(error = %e, "device_actions failed");
            Error::Api(e.to_string())
        })?;
        debug!(request_id = %result.request_id, "on/off response");
        Ok(())
    }

    pub async fn toggle_group(&self, group_id: &str, on: bool) -> Res<()> {
        debug!(group_id, on, "sending group on/off action");
        let request = GroupActionRequest {
            actions: vec![CapabilityAction::OnOff {
                state: OnOffState::On(on),
            }],
        };
        let result = self.0.group_actions(group_id, &request).await.map_err(|e| {
            error!(error = %e, "group_actions failed");
            Error::Api(e.to_string())
        })?;
        debug!(request_id = %result.request_id, "group on/off response");
        Ok(())
    }
}

fn color_action_to_state(action: ColorAction) -> ColorSettingState {
    match action {
        ColorAction::Rgb(rgb) => ColorSettingState::Rgb(rgb),
        ColorAction::Hsv { h, s, v } => ColorSettingState::Hsv(HsvColor { h, s, v }),
        ColorAction::Temperature(k) => ColorSettingState::TemperatureK(k),
    }
}
