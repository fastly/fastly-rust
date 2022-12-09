# LoggingGcsCommon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<**String**> | Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified. | 
**secret_key** | Option<**String**> | Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified. | 
**account_name** | Option<**String**> | The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


