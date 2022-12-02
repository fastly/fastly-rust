/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsCsrDataAttributes {
    /// Subject Altername Names - An array of one or more fully qualified domain names or public IP addresses to be secured by this certificate. Required.
    #[serde(rename = "sans")]
    pub sans: Vec<String>,
    /// Common Name (CN) - The fully qualified domain name (FQDN) to be secured by this certificate. The common name should be one of the entries in the SANs parameter.
    #[serde(rename = "common_name", skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// Country (C) - The two-letter ISO country code where the organization is located.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// State (S) - The state, province, region, or county where the organization is located. This should not be abbreviated.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Locality (L) - The locality, city, town, or village where the organization is located.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Postal Code - The postal code where the organization is located.
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Street Address - The street address where the organization is located.
    #[serde(rename = "street_address", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    /// Organization (O) - The legal name of the organization, including any suffixes. This should not be abbreviated.
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Organizational Unit (OU) - The internal division of the organization managing the certificate.
    #[serde(rename = "organizational_unit", skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    /// Email Address (EMAIL) - The organizational contact for this.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl TlsCsrDataAttributes {
    pub fn new(sans: Vec<String>) -> TlsCsrDataAttributes {
        TlsCsrDataAttributes {
            sans,
            common_name: None,
            country: None,
            state: None,
            city: None,
            postal_code: None,
            street_address: None,
            organization: None,
            organizational_unit: None,
            email: None,
        }
    }
}


