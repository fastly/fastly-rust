# VersionDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backends** | Option<[**Vec&lt;crate::models::BackendResponse&gt;**](BackendResponse.md)> | List of backends associated to this service. | 
**cache_settings** | Option<[**Vec&lt;crate::models::CacheSettingResponse&gt;**](CacheSettingResponse.md)> | List of cache settings associated to this service. | 
**conditions** | Option<[**Vec&lt;crate::models::ConditionResponse&gt;**](ConditionResponse.md)> | List of conditions associated to this service. | 
**directors** | Option<[**Vec&lt;crate::models::Director&gt;**](Director.md)> | List of directors associated to this service. | 
**domains** | Option<[**Vec&lt;crate::models::DomainResponse&gt;**](DomainResponse.md)> | List of domains associated to this service. | 
**gzips** | Option<[**Vec&lt;crate::models::GzipResponse&gt;**](GzipResponse.md)> | List of gzip rules associated to this service. | 
**headers** | Option<[**Vec&lt;crate::models::HeaderResponse&gt;**](HeaderResponse.md)> | List of headers associated to this service. | 
**healthchecks** | Option<[**Vec&lt;crate::models::HealthcheckResponse&gt;**](HealthcheckResponse.md)> | List of healthchecks associated to this service. | 
**request_settings** | Option<[**Vec&lt;crate::models::RequestSettingsResponse&gt;**](RequestSettingsResponse.md)> | List of request settings for this service. | 
**response_objects** | Option<[**Vec&lt;crate::models::ResponseObjectResponse&gt;**](ResponseObjectResponse.md)> | List of response objects for this service. | 
**settings** | Option<[**crate::models::VersionDetailSettings**](VersionDetailSettings.md)> |  | 
**snippets** | Option<[**Vec&lt;crate::models::SchemasSnippetResponse&gt;**](SchemasSnippetResponse.md)> | List of VCL snippets for this service. | 
**vcls** | Option<[**Vec&lt;crate::models::SchemasVclResponse&gt;**](SchemasVclResponse.md)> | List of VCL files for this service. | 
**wordpress** | Option<[**Vec&lt;serde_json::Value&gt;**](SerdeJsonValue.md)> | A list of Wordpress rules with this service. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


