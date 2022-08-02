# LoggingS3AllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key** | Option<**String**> | The access key for your S3 account. Not required if `iam_role` is provided. | 
**acl** | Option<**String**> | The access control list (ACL) specific request header. See the AWS documentation for [Access Control List (ACL) Specific Request Headers](https://docs.aws.amazon.com/AmazonS3/latest/API/mpUploadInitiate.html#initiate-mpu-acl-specific-request-headers) for more information. | 
**bucket_name** | Option<**String**> | The bucket name for S3 account. | 
**domain** | Option<**String**> | The domain of the Amazon S3 endpoint. | 
**iam_role** | Option<**String**> | The Amazon Resource Name (ARN) for the IAM role granting Fastly access to S3. Not required if `access_key` and `secret_key` are provided. | 
**path** | Option<**String**> | The path to upload logs to. | [default to null]
**public_key** | Option<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. | [default to null]
**redundancy** | Option<**String**> | The S3 redundancy level. | [default to null]
**secret_key** | Option<**String**> | The secret key for your S3 account. Not required if `iam_role` is provided. | 
**server_side_encryption_kms_key_id** | Option<**String**> | Optional server-side KMS Key Id. Must be set if `server_side_encryption` is set to `aws:kms` or `AES256`. | [default to null]
**server_side_encryption** | Option<**String**> | Set this to `AES256` or `aws:kms` to enable S3 Server Side Encryption. | [default to null]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


