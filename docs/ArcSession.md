# ArcSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the session. | [readonly]
**virtual_key_id** | **String** | The ID of the virtual key used for this session. | 
**virtual_key_name** | Option<**String**> | The display name of the virtual key used for this session. | 
**model** | **String** | The AI model identifier used. | 
**provider** | **String** | The name of the provider connection that ultimately served the session. | 
**provider_connection_id** | **String** | The ID of the provider connection that ultimately served the session. | 
**primary_provider** | Option<**String**> | The ID of the provider connection originally requested, before any failover occurred. | 
**requests** | **i32** | The number of requests made in this session. | 
**input_tokens** | **i32** | The total input tokens consumed in this session. | 
**output_tokens** | **i32** | The total output tokens generated in this session. | 
**violations** | **i32** | The number of AI Firewall violations detected in this session. | 
**firewall_enabled** | **bool** | Whether AI Firewall was enabled for this session's virtual key. | 
**firewall_outcome** | Option<**String**> | The outcome AI Firewall reached for this session. Present only when AI Firewall ran. | 
**firewall_stage** | Option<**String**> | The AI Firewall stage that produced the outcome (for example, the request or response stage). Present only when AI Firewall ran. | 
**firewall_policy** | Option<**String**> | The AI Firewall policy that was applied. Present only when AI Firewall ran. | 
**firewall_incomplete_reason** | Option<**String**> | Why AI Firewall inspection was incomplete, if applicable. Present only when AI Firewall ran and inspection didn't complete. | 
**created_at** | **String** | The date and time the session was created, in RFC 3339 format. | [readonly]
**updated_at** | **String** | The date and time the session was last updated, in RFC 3339 format. | [readonly]
**logs** | [**Vec&lt;crate::models::ArcSessionLogEntry&gt;**](ArcSessionLogEntry.md) | The request and response pairs recorded for this session. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


