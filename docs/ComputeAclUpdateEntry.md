# ComputeAclUpdateEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**op** | Option<**String**> | One of \"create\" or \"update\", indicating that the rest of this entry is to be added to/updated in the ACL. | 
**prefix** | Option<**String**> | An IP prefix defined in Classless Inter-Domain Routing (CIDR) format, i.e. a valid IP address (v4 or v6) followed by a forward slash (/) and a prefix length (0-32 or 0-128, depending on address family). | 
**action** | Option<**String**> | The action taken on the IP address, either \"block\" or \"allow\". | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


