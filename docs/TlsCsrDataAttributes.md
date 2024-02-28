# TlsCsrDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sans** | **Vec<String>** | Subject Alternate Names - An array of one or more fully qualified domain names or public IP addresses to be secured by this certificate. Required. | 
**common_name** | Option<**String**> | Common Name (CN) - The fully qualified domain name (FQDN) to be secured by this certificate. The common name should be one of the entries in the SANs parameter. | 
**country** | Option<**String**> | Country (C) - The two-letter ISO country code where the organization is located. | 
**state** | Option<**String**> | State (S) - The state, province, region, or county where the organization is located. This should not be abbreviated. | 
**city** | Option<**String**> | Locality (L) - The locality, city, town, or village where the organization is located. | 
**postal_code** | Option<**String**> | Postal Code - The postal code where the organization is located. | 
**street_address** | Option<**String**> | Street Address - The street address where the organization is located. | 
**organization** | Option<**String**> | Organization (O) - The legal name of the organization, including any suffixes. This should not be abbreviated. | 
**organizational_unit** | Option<**String**> | Organizational Unit (OU) - The internal division of the organization managing the certificate. | 
**email** | Option<**String**> | Email Address (EMAIL) - The organizational contact for this. | 
**key_type** | Option<**String**> | CSR Key Type. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


