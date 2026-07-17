# WafSimulateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**waf_response** | **i32** | The HTTP status code the WAF would return for the simulated request (e.g., `200` for allowed, `406` for blocked). | 
**signals** | [**Vec&lt;crate::models::WafSimulateSignal&gt;**](WafSimulateSignal.md) | List of signals detected by the WAF during simulation. Empty array when no signals are detected. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


