/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// OriginInspectorRealtimeEntryRecorded : The Unix timestamp at which this record's data was generated.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginInspectorRealtimeEntryRecorded {
}

impl OriginInspectorRealtimeEntryRecorded {
    /// The Unix timestamp at which this record's data was generated.
    pub fn new() -> OriginInspectorRealtimeEntryRecorded {
        OriginInspectorRealtimeEntryRecorded {
        }
    }
}


