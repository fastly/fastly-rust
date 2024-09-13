# OriginInspectorRealtimeEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recorded** | Option<**i32**> | The Unix timestamp at which this record's data was generated. | 
**aggregated** | Option<[**::std::collections::HashMap&lt;String, crate::models::OriginInspectorMeasurements&gt;**](OriginInspectorMeasurements.md)> | Groups [measurements](#measurements-data-model) by backend name. | 
**datacenter** | Option<[**::std::collections::HashMap&lt;String, ::std::collections::HashMap&lt;String, crate::models::OriginInspectorMeasurements&gt;&gt;**](Map.md)> | Groups [measurements](#measurements-data-model) by POP, then backend name. See the [POPs API](https://www.fastly.com/documentation/reference/api/utils/pops/) for details about POP identifiers. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


