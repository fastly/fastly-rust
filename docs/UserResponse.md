# UserResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**login** | Option<**String**> |  | [readonly]
**name** | Option<**String**> | The real life name of the user. | 
**limit_services** | Option<**bool**> | Indicates that the user has limited access to the customer's services. | 
**locked** | Option<**bool**> | Indicates whether the is account is locked for editing or not. | 
**require_new_password** | Option<**bool**> | Indicates if a new password is required at next login. | 
**role** | Option<[**crate::models::RoleUser**](RoleUser.md)> |  | 
**two_factor_auth_enabled** | Option<**bool**> | Indicates if 2FA is enabled on the user. | 
**two_factor_setup_required** | Option<**bool**> | Indicates if 2FA is required by the user's customer account. | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> |  | [readonly]
**email_hash** | Option<**String**> | The alphanumeric string identifying a email login. | [readonly]
**customer_id** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


