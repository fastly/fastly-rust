# LoggingBigqueryAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the BigQuery logging object. Used as a primary key for API access. | 
**format** | Option<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce JSON that matches the schema of your BigQuery table. | 
**dataset** | Option<**String**> | Your BigQuery dataset. | 
**table** | Option<**String**> | Your BigQuery table. | 
**template_suffix** | Option<**String**> | BigQuery table name suffix template. Optional. | 
**project_id** | Option<**String**> | Your Google Cloud Platform project ID. Required | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


