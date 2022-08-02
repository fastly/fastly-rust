/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VclDiff {
    /// The version number of the service to which changes in the generated VCL are being compared.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    /// The version number of the service from which changes in the generated VCL are being compared.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<i32>,
    /// The format in which compared VCL changes are being returned in.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    /// The differences between two specified versions.
    #[serde(rename = "diff", skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

impl VclDiff {
    pub fn new() -> VclDiff {
        VclDiff {
            from: None,
            to: None,
            format: None,
            diff: None,
        }
    }
}

/// The format in which compared VCL changes are being returned in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "html_simple")]
    HtmlSimple,
}

impl Default for Format {
    fn default() -> Format {
        Self::Text
    }
}

