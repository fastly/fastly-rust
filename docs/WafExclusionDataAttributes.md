# WafExclusionDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition** | Option<**String**> | A conditional expression in VCL used to determine if the condition is met. | 
**exclusion_type** | Option<**String**> | The type of exclusion. | 
**logging** | Option<**bool**> | Whether to generate a log upon matching. | [default to true]
**name** | Option<**String**> | Name of the exclusion. | 
**number** | Option<**i32**> | A numeric ID identifying a WAF exclusion. | 
**variable** | Option<**String**> | The variable to exclude. An optional selector can be specified after the variable separated by a colon (`:`) to restrict the variable to a particular parameter. Required for `exclusion_type=variable`. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


