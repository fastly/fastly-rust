# LoggingSftpAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**password** | Option<**String**> | The password for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference. | 
**path** | Option<**String**> | The path to upload logs to. | [default to null]
**port** | Option<**i32**> | The port number. | [default to 22]
**public_key** | Option<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. | [default to null]
**secret_key** | Option<**String**> | The SSH private key for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference. | [default to null]
**ssh_known_hosts** | Option<**String**> | A list of host keys for all hosts we can connect to over SFTP. | 
**user** | Option<**String**> | The username for the server. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


