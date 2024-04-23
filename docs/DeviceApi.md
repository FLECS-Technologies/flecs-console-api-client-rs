# \DeviceApi

All URIs are relative to *https://console.flecs.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_api_v2_device_license_activate**](DeviceApi.md#post_api_v2_device_license_activate) | **POST** /api/v2/device/license/activate | Activate license
[**post_api_v2_device_license_validate**](DeviceApi.md#post_api_v2_device_license_validate) | **POST** /api/v2/device/license/validate | Validate license



## post_api_v2_device_license_activate

> crate::models::PostApiV2DeviceLicenseActivate200Response post_api_v2_device_license_activate(authorization, x_session_id)
Activate license



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**x_session_id** | **String** |  | [required] |

### Return type

[**crate::models::PostApiV2DeviceLicenseActivate200Response**](post_api_v2_device_license_activate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_v2_device_license_validate

> crate::models::PostApiV2DeviceLicenseValidate200Response post_api_v2_device_license_validate(authorization, x_session_id)
Validate license



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**x_session_id** | **String** |  | [required] |

### Return type

[**crate::models::PostApiV2DeviceLicenseValidate200Response**](post_api_v2_device_license_validate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

