/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceResponse {
    /// Customer ID associated with the invoice.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Alphanumeric string identifying the invoice.
    #[serde(rename = "invoice_id", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Date and time invoice was posted on, in ISO 8601 format.
    #[serde(rename = "invoice_posted_on", skip_serializing_if = "Option::is_none")]
    pub invoice_posted_on: Option<String>,
    /// Date and time (in ISO 8601 format) for initiation point of a billing cycle, signifying the start of charges for a service or subscription.
    #[serde(rename = "billing_start_date", skip_serializing_if = "Option::is_none")]
    pub billing_start_date: Option<String>,
    /// Date and time (in ISO 8601 format) for termination point of a billing cycle, signifying the end of charges for a service or subscription.
    #[serde(rename = "billing_end_date", skip_serializing_if = "Option::is_none")]
    pub billing_end_date: Option<String>,
    /// Alphanumeric string identifying the statement number.
    #[serde(rename = "statement_number", skip_serializing_if = "Option::is_none")]
    pub statement_number: Option<String>,
    /// Three-letter code representing a specific currency used for financial transactions.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// Total billable amount for invoiced services charged within a single month.
    #[serde(rename = "monthly_transaction_amount", skip_serializing_if = "Option::is_none")]
    pub monthly_transaction_amount: Option<f32>,
    #[serde(rename = "transaction_line_items", skip_serializing_if = "Option::is_none")]
    pub transaction_line_items: Option<Vec<crate::models::Invoicelineitems>>,
}

impl InvoiceResponse {
    pub fn new() -> InvoiceResponse {
        InvoiceResponse {
            customer_id: None,
            invoice_id: None,
            invoice_posted_on: None,
            billing_start_date: None,
            billing_end_date: None,
            statement_number: None,
            currency_code: None,
            monthly_transaction_amount: None,
            transaction_line_items: None,
        }
    }
}


