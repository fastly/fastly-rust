/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingS3AllOf {
    /// The access key for your S3 account. Not required if `iam_role` is provided.
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// The access control list (ACL) specific request header. See the AWS documentation for [Access Control List (ACL) Specific Request Headers](https://docs.aws.amazon.com/AmazonS3/latest/API/mpUploadInitiate.html#initiate-mpu-acl-specific-request-headers) for more information.
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<String>,
    /// The bucket name for S3 account.
    #[serde(rename = "bucket_name", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The domain of the Amazon S3 endpoint.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The Amazon Resource Name (ARN) for the IAM role granting Fastly access to S3. Not required if `access_key` and `secret_key` are provided.
    #[serde(rename = "iam_role", skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// The path to upload logs to.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// The S3 redundancy level.
    #[serde(rename = "redundancy", skip_serializing_if = "Option::is_none")]
    pub redundancy: Option<String>,
    /// The secret key for your S3 account. Not required if `iam_role` is provided.
    #[serde(rename = "secret_key", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// Optional server-side KMS Key Id. Must be set if `server_side_encryption` is set to `aws:kms` or `AES256`.
    #[serde(rename = "server_side_encryption_kms_key_id", skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<String>,
    /// Set this to `AES256` or `aws:kms` to enable S3 Server Side Encryption.
    #[serde(rename = "server_side_encryption", skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
}

impl LoggingS3AllOf {
    pub fn new() -> LoggingS3AllOf {
        LoggingS3AllOf {
            access_key: None,
            acl: None,
            bucket_name: None,
            domain: None,
            iam_role: None,
            path: None,
            public_key: None,
            redundancy: None,
            secret_key: None,
            server_side_encryption_kms_key_id: None,
            server_side_encryption: None,
        }
    }
}


