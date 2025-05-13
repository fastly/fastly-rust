# ComputeAclUpdateEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**op** | Option<**String**> | One of \"create\", \"update\", or \"delete\" indicating the operation to perform on the update. | 
**prefix** | Option<**String**> | An IP prefix defined in Classless Inter-Domain Routing (CIDR) format, i.e. a valid IP address (v4 or v6) followed by a forward slash (/) and a prefix length (0-32 or 0-128, depending on address family). | 
**action** | Option<**String**> | The action taken on the IP address, one of \"BLOCK\" or \"ALLOW\". If using the \"delete\" operation, no action should be specified. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


