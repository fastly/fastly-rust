/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// RealtimeEntry : A list of records, each representing one second of time. The `Data` property provides access to [measurement data](#measurements-data-model) for that time period, grouped in various ways.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RealtimeEntry {
    #[serde(rename = "recorded", skip_serializing_if = "Option::is_none")]
    pub recorded: Option<Box<crate::models::RealtimeEntryRecorded>>,
    #[serde(rename = "aggregated", skip_serializing_if = "Option::is_none")]
    pub aggregated: Option<Box<crate::models::RealtimeEntryAggregated>>,
    /// Groups [measurements](#measurements-data-model) by POP. See the [POPs API](/reference/api/utils/pops/) for details of POP identifiers.
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<::std::collections::HashMap<String, crate::models::RealtimeMeasurements>>,
}

impl RealtimeEntry {
    /// A list of records, each representing one second of time. The `Data` property provides access to [measurement data](#measurements-data-model) for that time period, grouped in various ways.
    pub fn new() -> RealtimeEntry {
        RealtimeEntry {
            recorded: None,
            aggregated: None,
            datacenter: None,
        }
    }
}


