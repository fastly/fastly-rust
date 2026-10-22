/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectErrorDetail : A field-level validation error.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectErrorDetail {
    /// Request field associated with the error.
    #[serde(rename = "field")]
    pub field: String,
    /// Explanation of why the field is invalid.
    #[serde(rename = "error")]
    pub error: String,
}

impl NetworkProtectErrorDetail {
    /// A field-level validation error.
    pub fn new(field: String, error: String) -> NetworkProtectErrorDetail {
        NetworkProtectErrorDetail {
            field,
            error,
        }
    }
}


