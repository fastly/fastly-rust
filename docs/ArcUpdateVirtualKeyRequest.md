# ArcUpdateVirtualKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A display name for the virtual key. | 
**expires_at** | Option<**String**> | The date and time the key expires, in RFC 3339 format. Set explicitly to `null` to clear the expiration.  | 
**security_enabled** | Option<**bool**> | Whether AI Firewall security is enabled for requests made with this key. | 
**security_action** | Option<**String**> | The action AI Firewall takes when it detects a violation on a request made with this key. | 
**rpm_limit** | Option<**i32**> | The requests-per-minute limit for this key. Omit this field to leave the limit unchanged, set it to a positive integer to set a limit, or set it explicitly to `null` to remove the limit.  | 
**tpm_limit** | Option<**i32**> | The tokens-per-minute limit for this key. Omit this field to leave the limit unchanged, set it to a positive integer to set a limit, or set it explicitly to `null` to remove the limit.  | 
**spend_limit_enabled** | Option<**bool**> | Whether a spend limit is enabled for this key. This field cannot be set when the key is created; it can only be changed with this endpoint. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


