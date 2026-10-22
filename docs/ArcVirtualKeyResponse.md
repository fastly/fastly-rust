# ArcVirtualKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A display name for the virtual key. | 
**expires_at** | Option<**String**> | The date and time the key expires, in RFC 3339 format. Omit for a key that doesn't expire. | 
**security_enabled** | Option<**bool**> | Whether AI Firewall security is enabled for requests made with this key. | [default to false]
**security_action** | Option<**String**> | The action AI Firewall takes when it detects a violation on a request made with this key. Only meaningful when `security_enabled` is `true`. | 
**rpm_limit** | Option<**i32**> | The requests-per-minute limit for this key. Omit for no limit. | 
**tpm_limit** | Option<**i32**> | The tokens-per-minute limit for this key. Omit for no limit. | 
**id** | **String** | The unique identifier of the virtual key. | [readonly]
**_type** | **String** | The type of the resource. | [readonly]
**user_id** | **String** | The ID of the user who created the key. | [readonly]
**created_by** | Option<**String**> | The display name of the user who created the key. | [readonly]
**last_used_at** | Option<**String**> | The date and time the key was last used to make a request, in RFC 3339 format. `null` if the key has never been used. | [readonly]
**deleted_at** | Option<**String**> | The date and time the key was deleted, in RFC 3339 format. `null` unless the key was deleted and `include_deleted` was used to retrieve it. | [readonly]
**created_at** | **String** | The date and time the key was created, in RFC 3339 format. | [readonly]
**updated_at** | **String** | The date and time the key was last updated, in RFC 3339 format. | [readonly]
**spend_limit_enabled** | Option<**bool**> | Whether a spend limit is enabled for this key. | [readonly]
**spend_limit_enabled_on** | Option<**String**> | The date and time `spend_limit_enabled` was last set to `true`, in RFC 3339 format. `null` if the spend limit has never been enabled. This timestamp is not cleared when `spend_limit_enabled` is later set back to `false`, so it can be non-null even when the spend limit is currently disabled.  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


