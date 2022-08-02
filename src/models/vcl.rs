/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Vcl {
    /// The VCL code to be included.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Set to `true` when this is the main VCL, otherwise `false`.
    #[serde(rename = "main", skip_serializing_if = "Option::is_none")]
    pub main: Option<bool>,
    /// The name of this VCL.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Vcl {
    pub fn new() -> Vcl {
        Vcl {
            content: None,
            main: None,
            name: None,
        }
    }
}


