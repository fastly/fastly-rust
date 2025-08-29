# DdosProtectionRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> | Unique ID of the rule. | 
**name** | Option<**String**> | A human-readable name for the rule. | 
**action** | Option<**String**> | Action types for a rule. Supported action values are default, block, log, off. The default action value follows the current protection mode of the associated service. | [default to default]
**customer_id** | Option<**String**> | Alphanumeric string identifying the customer. | 
**service_id** | Option<**String**> | Alphanumeric string identifying the service. | 
**source_ip** | Option<**String**> | Source IP address attribute. | 
**country_code** | Option<**String**> | Country code attribute. | 
**host** | Option<**String**> | Host attribute. | 
**asn** | Option<**String**> | ASN attribute. | 
**source_ip_prefix** | Option<**String**> | Source IP prefix attribute. | 
**additional_attributes** | Option<**Vec<String>**> | Attribute category for additional, unlisted attributes used in a rule. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


