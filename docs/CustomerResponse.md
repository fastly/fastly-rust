# CustomerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**billing_contact_id** | Option<**String**> | The alphanumeric string representing the primary billing contact. | 
**billing_network_type** | Option<**String**> | Customer's current network revenue type. | 
**billing_ref** | Option<**String**> | Used for adding purchased orders to customer's account. | 
**can_configure_wordpress** | Option<**bool**> | Whether this customer can view or edit wordpress. | [readonly]
**can_reset_passwords** | Option<**bool**> | Whether this customer can reset passwords. | [readonly]
**can_upload_vcl** | Option<**bool**> | Whether this customer can upload VCL. | [readonly]
**force_2fa** | Option<**bool**> | Specifies whether 2FA is forced or not forced on the customer account. Logs out non-2FA users once 2FA is force enabled. | 
**force_sso** | Option<**bool**> | Specifies whether SSO is forced or not forced on the customer account. | 
**has_account_panel** | Option<**bool**> | Specifies whether the account has access or does not have access to the account panel. | 
**has_improved_events** | Option<**bool**> | Specifies whether the account has access or does not have access to the improved events. | 
**has_improved_ssl_config** | Option<**bool**> | Whether this customer can view or edit the SSL config. | [readonly]
**has_openstack_logging** | Option<**bool**> | Specifies whether the account has enabled or not enabled openstack logging. | 
**has_pci** | Option<**bool**> | Specifies whether the account can edit PCI for a service. | 
**has_pci_passwords** | Option<**bool**> | Specifies whether PCI passwords are required for the account. | [readonly]
**ip_whitelist** | Option<**String**> | The range of IP addresses authorized to access the customer account. | 
**legal_contact_id** | Option<**String**> | The alphanumeric string identifying the account's legal contact. | 
**name** | Option<**String**> | The name of the customer, generally the company name. | 
**owner_id** | Option<**String**> | The alphanumeric string identifying the account owner. | 
**phone_number** | Option<**String**> | The phone number associated with the account. | 
**postal_address** | Option<**String**> | The postal address associated with the account. | 
**pricing_plan** | Option<**String**> | The pricing plan this customer is under. | 
**pricing_plan_id** | Option<**String**> | The alphanumeric string identifying the pricing plan. | 
**security_contact_id** | Option<**String**> | The alphanumeric string identifying the account's security contact. | 
**technical_contact_id** | Option<**String**> | The alphanumeric string identifying the account's technical contact. | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


