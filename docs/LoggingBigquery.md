# LoggingBigquery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the BigQuery logging object. Used as a primary key for API access. | 
**placement** | Option<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  | 
**format_version** | Option<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  | [default to FormatVersion_v2]
**response_condition** | Option<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. | 
**format** | Option<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce JSON that matches the schema of your BigQuery table. | 
**user** | Option<**String**> | Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified. | 
**secret_key** | Option<**String**> | Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified. | 
**account_name** | Option<**String**> | The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided. | 
**dataset** | Option<**String**> | Your BigQuery dataset. | 
**table** | Option<**String**> | Your BigQuery table. | 
**template_suffix** | Option<**String**> | BigQuery table name suffix template. Optional. | 
**project_id** | Option<**String**> | Your Google Cloud Platform project ID. Required | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


