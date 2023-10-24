# ValuesDdos

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ddos_action_limit_streams_connections** | Option<**i32**> | For HTTP/2, the number of connections the limit-streams action was applied to. The limit-streams action caps the allowed number of concurrent streams in a connection. | 
**ddos_action_limit_streams_requests** | Option<**i32**> | For HTTP/2, the number of requests made on a connection for which the limit-streams action was taken. The limit-streams action caps the allowed number of concurrent streams in a connection. | 
**ddos_action_tarpit_accept** | Option<**i32**> | The number of times the tarpit-accept action was taken. The tarpit-accept action adds a delay when accepting future connections. | 
**ddos_action_tarpit** | Option<**i32**> | The number of times the tarpit action was taken. The tarpit action delays writing the response to the client. | 
**ddos_action_close** | Option<**i32**> | The number of times the close action was taken. The close action aborts the connection as soon as possible. The close action takes effect either right after accept, right after the client hello, or right after the response was sent. | 
**ddos_action_blackhole** | Option<**i32**> | The number of times the blackhole action was taken. The blackhole action quietly closes a TCP connection without sending a reset. The blackhole action quietly closes a TCP connection without notifying its peer (all TCP state is dropped). | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


