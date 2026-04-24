use crate::error::Res;
use crate::{
    ActionResponse, DeviceActionRequest, DeviceInfoResponse, Endpoint, GroupActionRequest,
    GroupInfoResponse, SimpleResponse, UserInfoResponse,
};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

/// Client for interacting with the Yandex Smart Home API
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    /// Creates a new client with the provided OAuth token
    pub fn new(token: String) -> Res<Self> {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|_| crate::error::Error::AuthToken)?;

        headers.insert(AUTHORIZATION, auth_value);

        let inner = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { inner })
    }

    /// Gets full info about the user's smart home setup
    pub async fn user_info(&self) -> Res<UserInfoResponse> {
        let endpoint = Endpoint::UserInfo;
        let resp = self
            .inner
            .request(endpoint.method(), endpoint.url()?)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Gets info about a specific device
    pub async fn device_info(&self, device_id: &str) -> Res<DeviceInfoResponse> {
        let endpoint = Endpoint::DeviceStatus {
            device_id: device_id.to_string(),
        };
        let resp = self
            .inner
            .request(endpoint.method(), endpoint.url()?)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Controls device actions
    pub async fn device_actions(&self, request: &DeviceActionRequest) -> Res<ActionResponse> {
        let endpoint = Endpoint::DeviceActions;
        let resp = self
            .inner
            .request(endpoint.method(), endpoint.url()?)
            .json(request)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Gets info about a specific group
    pub async fn group_info(&self, group_id: &str) -> Res<GroupInfoResponse> {
        let endpoint = Endpoint::GroupStatus {
            group_id: group_id.to_string(),
        };
        let resp = self
            .inner
            .request(endpoint.method(), endpoint.url()?)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Controls actions for a specific group
    pub async fn group_actions(
        &self,
        group_id: &str,
        request: &GroupActionRequest,
    ) -> Res<ActionResponse> {
        let endpoint = Endpoint::GroupActions {
            group_id: group_id.to_string(),
        };
        let resp = self
            .inner
            .request(endpoint.method(), endpoint.url()?)
            .json(request)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Triggers a specific scenario
    pub async fn scenario_actions(&self, scenario_id: &str) -> Res<SimpleResponse> {
        let endpoint = Endpoint::ScenarioActions {
            scenario_id: scenario_id.to_string(),
        };
        let resp = self
            .inner
            .request(endpoint.method(), endpoint.url()?)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Deletes a specific device
    pub async fn device_delete(&self, device_id: &str) -> Res<SimpleResponse> {
        let endpoint = Endpoint::DeviceDelete {
            device_id: device_id.to_string(),
        };
        let resp = self
            .inner
            .request(endpoint.method(), endpoint.url()?)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }
}
