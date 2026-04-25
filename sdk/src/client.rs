use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use tracing::{debug, error, instrument, warn};

use crate::error::{Error, Res};
use crate::simple_response::ResponseStatus;
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

        debug!("client initialised");
        Ok(Self { inner })
    }

    /// Returns the full smart home state for the authenticated user.
    ///
    /// Calls `GET /v1.0/user/info`.
    #[instrument(skip(self))]
    pub async fn user_info(&self) -> Res<UserInfoResponse> {
        let ep = Endpoint::UserInfo;
        let method = ep.method();
        let url = ep.url()?;
        debug!(%method, %url, "sending request");
        let response = self
            .inner
            .request(method, url)
            .send()
            .await
            .inspect_err(|e| error!(error = %e, "transport error"))?;
        debug!(status = %response.status(), "received response");
        Ok(response
            .json()
            .await
            .inspect_err(|e| error!(error = %e, "failed to deserialise response"))?)
    }

    /// Returns the current state of a single device.
    ///
    /// Calls `GET /v1.0/devices/{device_id}`.
    #[instrument(skip(self))]
    pub async fn device_info(&self, device_id: &str) -> Res<DeviceInfoResponse> {
        let ep = Endpoint::DeviceStatus { device_id: device_id.to_string() };
        let method = ep.method();
        let url = ep.url()?;
        debug!(%method, %url, "sending request");
        let response = self
            .inner
            .request(method, url)
            .send()
            .await
            .inspect_err(|e| error!(error = %e, "transport error"))?;
        debug!(status = %response.status(), "received response");
        Ok(response
            .json()
            .await
            .inspect_err(|e| error!(error = %e, "failed to deserialise response"))?)
    }

    /// Sends capability actions to one or more devices.
    ///
    /// Calls `POST /v1.0/devices/actions`.
    #[instrument(skip(self, request), fields(device_count = request.devices.len()))]
    pub async fn device_actions(&self, request: &DeviceActionRequest) -> Res<ActionResponse> {
        let ep = Endpoint::DeviceActions;
        let method = ep.method();
        let url = ep.url()?;
        debug!(%method, %url, "sending request");
        let response = self
            .inner
            .request(method, url)
            .json(request)
            .send()
            .await
            .inspect_err(|e| error!(error = %e, "transport error"))?;
        let status = response.status();
        debug!(%status, "received response");
        let text = response
            .text()
            .await
            .inspect_err(|e| error!(error = %e, "failed to read response body"))?;
        debug!(body = %text, "raw response body");
        let body: ActionResponse = serde_json::from_str(&text)
            .inspect_err(|e| error!(error = %e, "failed to deserialise response"))?;
        if body.status == ResponseStatus::Error {
            warn!(request_id = %body.request_id, "API returned error status");
        } else {
            debug!(request_id = %body.request_id, "request succeeded");
        }
        Ok(body)
    }

    /// Returns the current state of a device group.
    ///
    /// Calls `GET /v1.0/groups/{group_id}`.
    #[instrument(skip(self))]
    pub async fn group_info(&self, group_id: &str) -> Res<GroupInfoResponse> {
        let ep = Endpoint::GroupStatus { group_id: group_id.to_string() };
        let method = ep.method();
        let url = ep.url()?;
        debug!(%method, %url, "sending request");
        let response = self
            .inner
            .request(method, url)
            .send()
            .await
            .inspect_err(|e| error!(error = %e, "transport error"))?;
        debug!(status = %response.status(), "received response");
        Ok(response
            .json()
            .await
            .inspect_err(|e| error!(error = %e, "failed to deserialise response"))?)
    }

    /// Sends capability actions to every device in a group.
    ///
    /// Calls `POST /v1.0/groups/{group_id}/actions`.
    #[instrument(skip(self, request), fields(action_count = request.actions.len()))]
    pub async fn group_actions(
        &self,
        group_id: &str,
        request: &GroupActionRequest,
    ) -> Res<ActionResponse> {
        let ep = Endpoint::GroupActions { group_id: group_id.to_string() };
        let method = ep.method();
        let url = ep.url()?;
        debug!(%method, %url, "sending request");
        let response = self
            .inner
            .request(method, url)
            .json(request)
            .send()
            .await
            .inspect_err(|e| error!(error = %e, "transport error"))?;
        debug!(status = %response.status(), "received response");
        let body: ActionResponse = response
            .json()
            .await
            .inspect_err(|e| error!(error = %e, "failed to deserialise response"))?;
        if body.status == ResponseStatus::Error {
            warn!(request_id = %body.request_id, "API returned error status");
        } else {
            debug!(request_id = %body.request_id, "request succeeded");
        }
        Ok(body)
    }

    /// Triggers a user-defined scenario by its ID.
    ///
    /// Calls `POST /v1.0/scenarios/{scenario_id}/actions`.
    #[instrument(skip(self))]
    pub async fn scenario_trigger(&self, scenario_id: &str) -> Res<SimpleResponse> {
        let ep = Endpoint::ScenarioActions { scenario_id: scenario_id.to_string() };
        let method = ep.method();
        let url = ep.url()?;
        debug!(%method, %url, "sending request");
        let response = self
            .inner
            .request(method, url)
            .send()
            .await
            .inspect_err(|e| error!(error = %e, "transport error"))?;
        debug!(status = %response.status(), "received response");
        let body: SimpleResponse = response
            .json()
            .await
            .inspect_err(|e| error!(error = %e, "failed to deserialise response"))?;
        if body.status == ResponseStatus::Error {
            warn!(request_id = %body.request_id, "API returned error status");
        } else {
            debug!(request_id = %body.request_id, "request succeeded");
        }
        Ok(body)
    }

    /// Permanently removes a device from the user's account.
    ///
    /// Calls `DELETE /v1.0/devices/{device_id}`.
    #[instrument(skip(self))]
    pub async fn device_delete(&self, device_id: &str) -> Res<SimpleResponse> {
        let ep = Endpoint::DeviceDelete { device_id: device_id.to_string() };
        let method = ep.method();
        let url = ep.url()?;
        debug!(%method, %url, "sending request");
        let response = self
            .inner
            .request(method, url)
            .send()
            .await
            .inspect_err(|e| error!(error = %e, "transport error"))?;
        debug!(status = %response.status(), "received response");
        let body: SimpleResponse = response
            .json()
            .await
            .inspect_err(|e| error!(error = %e, "failed to deserialise response"))?;
        if body.status == ResponseStatus::Error {
            warn!(request_id = %body.request_id, "API returned error status");
        } else {
            debug!(request_id = %body.request_id, "request succeeded");
        }
        Ok(body)
    }
}
