# LoggingKinesis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the real-time logging configuration. | 
**placement** | Option<[**crate::models::LoggingPlacement**](LoggingPlacement.md)> |  | 
**format_version** | Option<[**crate::models::LoggingFormatVersion**](LoggingFormatVersion.md)> |  | 
**format** | Option<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce valid JSON that Kinesis can ingest. | [default to {"timestamp":"%{begin:%Y-%m-%dT%H:%M:%S}t","time_elapsed":"%{time.elapsed.usec}V","is_tls":"%{if(req.is_ssl, \"true\", \"false\")}V","client_ip":"%{req.http.Fastly-Client-IP}V","geo_city":"%{client.geo.city}V","geo_country_code":"%{client.geo.country_code}V","request":"%{req.request}V","host":"%{req.http.Fastly-Orig-Host}V","url":"%{json.escape(req.url)}V","request_referer":"%{json.escape(req.http.Referer)}V","request_user_agent":"%{json.escape(req.http.User-Agent)}V","request_accept_language":"%{json.escape(req.http.Accept-Language)}V","request_accept_charset":"%{json.escape(req.http.Accept-Charset)}V","cache_status":"%{regsub(fastly_info.state, \"^(HIT-(SYNTH)|(HITPASS|HIT|MISS|PASS|ERROR|PIPE)).*\", \"\\2\\3\") }V"}]
**topic** | Option<**String**> | The Amazon Kinesis stream to send logs to. Required. | 
**region** | Option<[**crate::models::AwsRegion**](AwsRegion.md)> |  | 
**secret_key** | Option<**String**> | The secret key associated with the target Amazon Kinesis stream. Not required if `iam_role` is specified. | 
**access_key** | Option<**String**> | The access key associated with the target Amazon Kinesis stream. Not required if `iam_role` is specified. | 
**iam_role** | Option<**String**> | The ARN for an IAM role granting Fastly access to the target Amazon Kinesis stream. Not required if `access_key` and `secret_key` are provided. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


