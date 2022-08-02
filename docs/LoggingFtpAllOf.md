# LoggingFtpAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | An hostname or IPv4 address. | 
**hostname** | Option<**String**> | Hostname used. | 
**ipv4** | Option<**String**> | IPv4 address of the host. | 
**password** | Option<**String**> | The password for the server. For anonymous use an email address. | 
**path** | Option<**String**> | The path to upload log files to. If the path ends in `/` then it is treated as a directory. | 
**port** | Option<**i32**> | The port number. | [default to 21]
**public_key** | Option<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. | [default to null]
**user** | Option<**String**> | The username for the server. Can be anonymous. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


