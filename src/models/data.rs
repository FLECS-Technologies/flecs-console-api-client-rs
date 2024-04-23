/*
 * FLECS Console API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "feature_flags", skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Box<crate::models::FeatureFlags>>,
    #[serde(rename = "jwt", skip_serializing_if = "Option::is_none")]
    pub jwt: Option<Box<crate::models::Jwt>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            feature_flags: None,
            jwt: None,
            user: None,
        }
    }
}