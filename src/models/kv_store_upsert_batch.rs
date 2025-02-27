/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KvStoreUpsertBatch {
    /// The key of the item to be added, updated, retrieved, or deleted.
    #[serde(rename = "key")]
    pub key: String,
    /// Indicates that the item should be deleted after the specified number of seconds have elapsed. Deletion is not immediate but will occur within 24 hours of the requested expiration.
    #[serde(rename = "time_to_live_sec", skip_serializing_if = "Option::is_none")]
    pub time_to_live_sec: Option<i32>,
    /// An arbitrary data field which can contain up to 2000 bytes of data.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// If set to true, the new value for the item will not be immediately visible to other users of the KV store; they will receive the existing (stale) value while the platform updates cached copies. Setting this to true ensures that other users of the KV store will receive responses to 'get' operations for this item quickly, although they will be slightly out of date.
    #[serde(rename = "background_fetch", skip_serializing_if = "Option::is_none")]
    pub background_fetch: Option<bool>,
    /// Value for the item (in Base64 encoding).
    #[serde(rename = "value")]
    pub value: String,
}

impl KvStoreUpsertBatch {
    pub fn new(key: String, value: String) -> KvStoreUpsertBatch {
        KvStoreUpsertBatch {
            key,
            time_to_live_sec: None,
            metadata: None,
            background_fetch: None,
            value,
        }
    }
}


