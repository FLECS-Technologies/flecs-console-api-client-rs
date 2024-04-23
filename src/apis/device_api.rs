/*
 * FLECS Console API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed successes of method [`post_api_v2_device_license_activate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostApiV2DeviceLicenseActivateSuccess {
    Status200(crate::models::PostApiV2DeviceLicenseActivate200Response),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_api_v2_device_license_validate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostApiV2DeviceLicenseValidateSuccess {
    Status200(crate::models::PostApiV2DeviceLicenseValidate200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_api_v2_device_license_activate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostApiV2DeviceLicenseActivateError {
    Status403(crate::models::ErrorDescription),
    Status500(crate::models::ErrorDescription),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_api_v2_device_license_validate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostApiV2DeviceLicenseValidateError {
    Status403(crate::models::ErrorDescription),
    Status500(crate::models::ErrorDescription),
    UnknownValue(serde_json::Value),
}

///
pub async fn post_api_v2_device_license_activate(
    configuration: &configuration::Configuration,
    authorization: &str,
    x_session_id: &str,
) -> Result<
    ResponseContent<PostApiV2DeviceLicenseActivateSuccess>,
    Error<PostApiV2DeviceLicenseActivateError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v2/device/license/activate",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Session-Id", x_session_id.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PostApiV2DeviceLicenseActivateSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PostApiV2DeviceLicenseActivateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn post_api_v2_device_license_validate(
    configuration: &configuration::Configuration,
    authorization: &str,
    x_session_id: &str,
) -> Result<
    ResponseContent<PostApiV2DeviceLicenseValidateSuccess>,
    Error<PostApiV2DeviceLicenseValidateError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v2/device/license/validate",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Session-Id", x_session_id.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PostApiV2DeviceLicenseValidateSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PostApiV2DeviceLicenseValidateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
