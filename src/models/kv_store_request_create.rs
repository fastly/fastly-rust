/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KvStoreRequestCreate {
    /// A human-readable name for the store. Refer to https://docs.fastly.com/products/compute-resource-limits#kv-store for limitations on the KV store name.
    #[serde(rename = "name")]
    pub name: String,
}

impl KvStoreRequestCreate {
    pub fn new(name: String) -> KvStoreRequestCreate {
        KvStoreRequestCreate {
            name,
        }
    }
}


