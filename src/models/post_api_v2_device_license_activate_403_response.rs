/*
 * New Spec - OpenAPI 3.0
 *
 * This is a sample description about this spec.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostApiV2DeviceLicenseActivate403Response {
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "statusText", skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
}

impl PostApiV2DeviceLicenseActivate403Response {
    pub fn new() -> PostApiV2DeviceLicenseActivate403Response {
        PostApiV2DeviceLicenseActivate403Response {
            reason: None,
            status_code: None,
            status_text: None,
        }
    }
}


