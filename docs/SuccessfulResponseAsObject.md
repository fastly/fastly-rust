# SuccessfulResponseAsObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> | Domain Identifier (UUID). | 
**fqdn** | Option<**String**> | The fully-qualified domain name for your domain. Can be created, but not updated. | 
**service_id** | Option<**String**> | The `service_id` associated with your domain or `null` if there is no association. | 
**description** | Option<**String**> | A freeform descriptive note. | 
**activated** | Option<**bool**> | Denotes if the domain has at least one TLS activation or not. | [readonly]
**verified** | Option<**bool**> | Denotes that the customer has proven ownership over the domain by obtaining a Fastly-managed TLS certificate for it or by providing a their own TLS certificate from a publicly-trusted CA that includes the domain or matching wildcard.      | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


