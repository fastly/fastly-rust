# ArcUpdateVirtualKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the virtual key. | [readonly]
**customer_id** | **String** | The ID of the customer account that owns the key. | [readonly]
**name** | **String** | The display name of the virtual key. | 
**user_id** | **String** | The ID of the user who created the key. | [readonly]
**expires_at** | Option<**String**> | The date and time the key expires, in RFC 3339 format. `null` if the key doesn't expire. | 
**rpm_limit** | Option<**i32**> | The requests-per-minute limit for this key. `null` if there is no limit. | 
**tpm_limit** | Option<**i32**> | The tokens-per-minute limit for this key. `null` if there is no limit. | 
**spend_limit_enabled** | Option<**bool**> | Whether a spend limit is enabled for this key. | [readonly]
**spend_limit_enabled_on** | Option<**String**> | The date and time `spend_limit_enabled` was last set to `true`, in RFC 3339 format. `null` if the spend limit has never been enabled. This timestamp is not cleared when `spend_limit_enabled` is later set back to `false`, so it can be non-null even when the spend limit is currently disabled.  | [readonly]
**created_at** | **String** | The date and time the key was created, in RFC 3339 format. | [readonly]
**updated_at** | **String** | The date and time the key was last updated, in RFC 3339 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


