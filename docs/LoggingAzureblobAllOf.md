# LoggingAzureblobAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | Option<**String**> | The path to upload logs to. | [default to null]
**account_name** | Option<**String**> | The unique Azure Blob Storage namespace in which your data objects are stored. Required. | 
**container** | Option<**String**> | The name of the Azure Blob Storage container in which to store logs. Required. | 
**sas_token** | Option<**String**> | The Azure shared access signature providing write access to the blob service objects. Be sure to update your token before it expires or the logging functionality will not work. Required. | 
**public_key** | Option<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. | [default to null]
**file_max_bytes** | Option<**i32**> | The maximum number of bytes for each uploaded file. A value of 0 can be used to indicate there is no limit on the size of uploaded files, otherwise the minimum value is 1048576 bytes (1 MiB.) | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


