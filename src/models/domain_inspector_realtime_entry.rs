/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// DomainInspectorRealtimeEntry : Each reporting period is represented by an entry in the `Data` property of the top level response and provides access to [measurement data](#measurements-data-model) for that time period, grouped in various ways: by domain name, domain IP address, and optionally by POP. The `datacenter` property organizes the measurements by Fastly POP, while the `aggregated` property combines the measurements of all POPs (but still splits by backend name and IP). 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainInspectorRealtimeEntry {
    #[serde(rename = "recorded", skip_serializing_if = "Option::is_none")]
    pub recorded: Option<Box<crate::models::RecordedTimestamp>>,
    /// Groups [measurements](#measurements-data-model) by backend name and then by IP address.
    #[serde(rename = "aggregated", skip_serializing_if = "Option::is_none")]
    pub aggregated: Option<::std::collections::HashMap<String, crate::models::DomainInspectorMeasurements>>,
    /// Groups [measurements](#measurements-data-model) by POP, then backend name, and then IP address. See the [POPs API](/reference/api/utils/pops/) for details about POP identifiers.
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, crate::models::DomainInspectorMeasurements>>>,
}

impl DomainInspectorRealtimeEntry {
    /// Each reporting period is represented by an entry in the `Data` property of the top level response and provides access to [measurement data](#measurements-data-model) for that time period, grouped in various ways: by domain name, domain IP address, and optionally by POP. The `datacenter` property organizes the measurements by Fastly POP, while the `aggregated` property combines the measurements of all POPs (but still splits by backend name and IP). 
    pub fn new() -> DomainInspectorRealtimeEntry {
        DomainInspectorRealtimeEntry {
            recorded: None,
            aggregated: None,
            datacenter: None,
        }
    }
}


