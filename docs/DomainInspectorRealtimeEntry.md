# DomainInspectorRealtimeEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recorded** | Option<[**crate::models::RecordedTimestamp**](RecordedTimestamp.md)> |  | 
**aggregated** | Option<[**::std::collections::HashMap&lt;String, crate::models::DomainInspectorMeasurements&gt;**](DomainInspectorMeasurements.md)> | Groups [measurements](#measurements-data-model) by backend name and then by IP address. | 
**datacenter** | Option<[**::std::collections::HashMap&lt;String, ::std::collections::HashMap&lt;String, crate::models::DomainInspectorMeasurements&gt;&gt;**](Map.md)> | Groups [measurements](#measurements-data-model) by POP, then backend name, and then IP address. See the [POPs API](/reference/api/utils/pops/) for details about POP identifiers. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


