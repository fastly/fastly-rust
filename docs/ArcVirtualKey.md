# ArcVirtualKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A display name for the virtual key. | 
**expires_at** | Option<**String**> | The date and time the key expires, in RFC 3339 format. Omit for a key that doesn't expire. | 
**security_enabled** | Option<**bool**> | Whether AI Firewall security is enabled for requests made with this key. | [default to false]
**security_action** | Option<**String**> | The action AI Firewall takes when it detects a violation on a request made with this key. Only meaningful when `security_enabled` is `true`. | 
**rpm_limit** | Option<**i32**> | The requests-per-minute limit for this key. Omit for no limit. | 
**tpm_limit** | Option<**i32**> | The tokens-per-minute limit for this key. Omit for no limit. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


