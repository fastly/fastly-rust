# OriginInspectorRealtimeEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recorded** | Option<[**crate::models::OriginInspectorRealtimeEntryRecorded**](OriginInspectorRealtimeEntryRecorded.md)> |  | 
**aggregated** | Option<[**::std::collections::HashMap&lt;String, crate::models::OriginInspectorMeasurements&gt;**](OriginInspectorMeasurements.md)> | Groups [measurements](#measurements-data-model) by backend name. | 
**datacenter** | Option<[**::std::collections::HashMap&lt;String, ::std::collections::HashMap&lt;String, crate::models::OriginInspectorMeasurements&gt;&gt;**](Map.md)> | Groups [measurements](#measurements-data-model) by POP, then backend name. See the [POPs API](/reference/api/utils/pops/) for details about POP identifiers. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


