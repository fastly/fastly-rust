# SetConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workspace_id** | Option<**String**> | The new workspace_id. Required in the `PUT` request body when `product_id` is `ngwaf`. Optional in the `PATCH` request body for `ngwaf`. | 
**traffic_ramp** | Option<**String**> | The new traffic ramp. Optional in the `PATCH` request body for `ngwaf`. | 
**mode** | Option<**String**> | The new mode to run the product in. One of `block`, `log`, or `off`. Optional in the `PATCH` request body for `ddos_protection`. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


