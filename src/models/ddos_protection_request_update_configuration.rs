/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DdosProtectionRequestUpdateConfiguration {
    /// Operation mode
    #[serde(rename = "mode")]
    pub mode: Mode,
}

impl DdosProtectionRequestUpdateConfiguration {
    pub fn new(mode: Mode) -> DdosProtectionRequestUpdateConfiguration {
        DdosProtectionRequestUpdateConfiguration {
            mode,
        }
    }
}

/// Operation mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "false")]
    _false,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "block")]
    Block,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::_false
    }
}

