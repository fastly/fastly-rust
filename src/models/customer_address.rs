/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomerAddress {
    /// The type of the address.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The street number and name of the address.
    #[serde(rename = "address_1", skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Additional address line for unit number, P.O. Box, etc.
    #[serde(rename = "address_2", skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// City, town, or locality name the address is located.
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// State, province, or region of the address.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// ISO 3166-1 alpha-2 country code (e.g., \"US\", \"CA\", \"NZ\")
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Postal or Zip code of the address.
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl CustomerAddress {
    pub fn new() -> CustomerAddress {
        CustomerAddress {
            _type: None,
            address_1: None,
            address_2: None,
            locality: None,
            region: None,
            country: None,
            postal_code: None,
        }
    }
}


