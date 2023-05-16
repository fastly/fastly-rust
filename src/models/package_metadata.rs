/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// PackageMetadata : [Package metadata](#metadata-model) that has been extracted from the uploaded package. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Name of the Compute@Edge package.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the Compute@Edge package.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of package authors' email addresses.
    #[serde(rename = "authors", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    /// The language of the Compute@Edge package.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Size of the Compute@Edge package in bytes.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Hash of the Compute@Edge package.
    #[serde(rename = "hashsum", skip_serializing_if = "Option::is_none")]
    pub hashsum: Option<String>,
    /// Hash of the files within the Compute@Edge package.
    #[serde(rename = "files_hash", skip_serializing_if = "Option::is_none")]
    pub files_hash: Option<String>,
}

impl PackageMetadata {
    /// [Package metadata](#metadata-model) that has been extracted from the uploaded package. 
    pub fn new() -> PackageMetadata {
        PackageMetadata {
            name: None,
            description: None,
            authors: None,
            language: None,
            size: None,
            hashsum: None,
            files_hash: None,
        }
    }
}


