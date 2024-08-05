/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Invoicelineitems {
    /// Invoice line item transaction name.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Billed amount for line item.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Discount coupon associated with the invoice for any account or service credits.
    #[serde(rename = "credit_coupon_code", skip_serializing_if = "Option::is_none")]
    pub credit_coupon_code: Option<String>,
    /// Price per unit.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    /// Total number of units of usage.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<f32>,
    /// The name of the product.
    #[serde(rename = "product_name", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// The broader classification of the product (e.g., `Compute` or `Full-Site Delivery`).
    #[serde(rename = "product_group", skip_serializing_if = "Option::is_none")]
    pub product_group: Option<String>,
    /// The broader classification of the product (e.g., `Network Services` or `Security`).
    #[serde(rename = "product_line", skip_serializing_if = "Option::is_none")]
    pub product_line: Option<String>,
    /// The geographical area applicable for regionally based products.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The unit of measure (e.g., `requests` or `bandwidth`).
    #[serde(rename = "usage_type", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

impl Invoicelineitems {
    pub fn new() -> Invoicelineitems {
        Invoicelineitems {
            description: None,
            amount: None,
            credit_coupon_code: None,
            rate: None,
            units: None,
            product_name: None,
            product_group: None,
            product_line: None,
            region: None,
            usage_type: None,
        }
    }
}


