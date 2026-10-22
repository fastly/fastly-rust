# ArcVirtualKeyFailoverRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**targets** | [**Vec&lt;crate::models::ArcFailoverTarget&gt;**](ArcFailoverTarget.md) | The provider connections to retry against, in order. | 
**timeout_enabled** | Option<**bool**> | Whether a request timeout triggers failover. | 
**rate_limit_429_enabled** | Option<**bool**> | Whether a `429 Too Many Requests` response from the provider triggers failover. | 
**server_error_5xx_enabled** | Option<**bool**> | Whether a `5xx` server error response from the provider triggers failover. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


