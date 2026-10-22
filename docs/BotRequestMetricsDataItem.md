# BotRequestMetricsDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | Option<**String**> | Timestamp for the time series being reported. | 
**bot** | Option<**String**> | Bot identifier for which data was recorded. Present only when grouped by bot. | 
**datacenter** | Option<**String**> | Datacenter (POP) for which data was recorded. Present only when grouped by datacenter. | 
**bot_edge_requests_allowed_count** | Option<**i64**> | Count of edge requests where a bot was detected and the request was allowed by the configured policy. | 
**bot_edge_requests_challenged_count** | Option<**i64**> | Count of edge requests where a bot was detected and a client challenge was issued for the request by the configured policy. | 
**bot_edge_requests_blocked_count** | Option<**i64**> | Count of edge requests where a bot was detected and the request was blocked by the configured policy. | 
**bot_edge_requests_challenge_token_valid_count** | Option<**i64**> | Count of edge requests where a bot was detected and the client making the request has already solved a challenge issued by the configured policy. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


