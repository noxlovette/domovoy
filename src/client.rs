use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

use crate::error::{Error, Res};
use crate::{
    ActionResponse, DeviceActionRequest, DeviceInfoResponse, Endpoint, GroupActionRequest,
    GroupInfoResponse, SimpleResponse, UserInfoResponse,
};

/// Client for the Yandex Smart Home API
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    /// Creates a new client authenticated with the given OAuth token.
    ///
    /// Returns `Err` if the token string contains characters that are not
    /// valid in an HTTP header value.
    pub fn new(token: impl Into<String>) -> Res<Self> {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", token.into()))
            .map_err(|_| Error::AuthToken)?;
        headers.insert(AUTHORIZATION, auth_value);

        let inner = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { inner })
    }

    /// Returns the full smart home state for the authenticated user.
    ///
    /// Calls `GET /v1.0/user/info`.
    pub async fn user_info(&self) -> Res<UserInfoResponse> {
        let ep = Endpoint::UserInfo;
        Ok(self.inner.request(ep.method(), ep.url()?).send().await?.json().await?)
    }

    /// Returns the current state of a single device.
    ///
    /// Calls `GET /v1.0/devices/{device_id}`.
    pub async fn device_info(&self, device_id: &str) -> Res<DeviceInfoResponse> {
        let ep = Endpoint::DeviceStatus { device_id: device_id.to_string() };
        Ok(self.inner.request(ep.method(), ep.url()?).send().await?.json().await?)
    }

    /// Sends capability actions to one or more devices.
    ///
    /// Calls `POST /v1.0/devices/actions`.
    pub async fn device_actions(&self, request: &DeviceActionRequest) -> Res<ActionResponse> {
        let ep = Endpoint::DeviceActions;
        Ok(self.inner.request(ep.method(), ep.url()?).json(request).send().await?.json().await?)
    }

    /// Returns the current state of a device group.
    ///
    /// Calls `GET /v1.0/groups/{group_id}`.
    pub async fn group_info(&self, group_id: &str) -> Res<GroupInfoResponse> {
        let ep = Endpoint::GroupStatus { group_id: group_id.to_string() };
        Ok(self.inner.request(ep.method(), ep.url()?).send().await?.json().await?)
    }

    /// Sends capability actions to every device in a group.
    ///
    /// Calls `POST /v1.0/groups/{group_id}/actions`.
    pub async fn group_actions(
        &self,
        group_id: &str,
        request: &GroupActionRequest,
    ) -> Res<ActionResponse> {
        let ep = Endpoint::GroupActions { group_id: group_id.to_string() };
        Ok(self.inner.request(ep.method(), ep.url()?).json(request).send().await?.json().await?)
    }

    /// Triggers a user-defined scenario by its ID.
    ///
    /// Calls `POST /v1.0/scenarios/{scenario_id}/actions`.
    pub async fn scenario_trigger(&self, scenario_id: &str) -> Res<SimpleResponse> {
        let ep = Endpoint::ScenarioActions { scenario_id: scenario_id.to_string() };
        Ok(self.inner.request(ep.method(), ep.url()?).send().await?.json().await?)
    }

    /// Permanently removes a device from the user's account.
    ///
    /// Calls `DELETE /v1.0/devices/{device_id}`.
    pub async fn device_delete(&self, device_id: &str) -> Res<SimpleResponse> {
        let ep = Endpoint::DeviceDelete { device_id: device_id.to_string() };
        Ok(self.inner.request(ep.method(), ep.url()?).send().await?.json().await?)
    }
}
