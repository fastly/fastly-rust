# RoutingConfigResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Alphanumeric string identifying the routing config. | [readonly]
**name** | Option<**String**> | The user-defined name for the routing config. | 
**state** | Option<[**crate::models::RoutingConfigState**](RoutingConfigState.md)> |  | 
**activated_at** | Option<**String**> | Timestamp of when the version was most recently activated. `null` if the version has never been activated. | [readonly]
**links** | Option<**::std::collections::HashMap<String, String>**> | HATEOAS links to related resources. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


