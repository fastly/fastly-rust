# AclEntryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**negated** | Option<**i32**> | Whether to negate the match. Useful primarily when creating individual exceptions to larger subnets. | [default to Negated_disable]
**comment** | Option<**String**> | A freeform descriptive note. | 
**ip** | Option<**String**> | An IP address. | 
**subnet** | Option<**i32**> | Number of bits for the subnet mask applied to the IP address. For IPv4 addresses, a value of 32 represents the smallest subnet mask (1 address), 24 represents a class C subnet mask (256 addresses), 16 represents a class B subnet mask (65k addresses), and 8 is class A subnet mask (16m addresses). If not provided, no mask is applied. | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**acl_id** | Option<**String**> |  | [readonly]
**id** | Option<**String**> |  | [readonly]
**service_id** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


