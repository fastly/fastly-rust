# LoggingKafkaAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**topic** | Option<**String**> | The Kafka topic to send logs to. Required. | 
**brokers** | Option<**String**> | A comma-separated list of IP addresses or hostnames of Kafka brokers. Required. | 
**compression_codec** | Option<**String**> | The codec used for compression of your logs. | 
**required_acks** | Option<**i32**> | The number of acknowledgements a leader must receive before a write is considered successful. | [default to RequiredAcks_one]
**request_max_bytes** | Option<**i32**> | The maximum number of bytes sent in one request. Defaults `0` (no limit). | [default to 0]
**parse_log_keyvals** | Option<**bool**> | Enables parsing of key=value tuples from the beginning of a logline, turning them into [record headers](https://cwiki.apache.org/confluence/display/KAFKA/KIP-82+-+Add+Record+Headers). | 
**auth_method** | Option<**String**> | SASL authentication method. | 
**user** | Option<**String**> | SASL user. | 
**password** | Option<**String**> | SASL password. | 
**use_tls** | Option<[**crate::models::LoggingUseTls**](LoggingUseTls.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


