/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AiAcceleratorResponseCustomer {
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<crate::models::AiAcceleratorResponseCustomerCustomer>>,
}

impl AiAcceleratorResponseCustomer {
    pub fn new() -> AiAcceleratorResponseCustomer {
        AiAcceleratorResponseCustomer {
            customer: None,
        }
    }
}


