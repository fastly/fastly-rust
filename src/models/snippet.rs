/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Snippet {
    /// The name for the snippet.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the snippet version.
    #[serde(rename = "dynamic", skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<Dynamic>,
    /// The location in generated VCL where the snippet should be placed.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The VCL code that specifies exactly what the snippet does.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Priority determines execution order. Lower numbers execute first.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

impl Snippet {
    pub fn new() -> Snippet {
        Snippet {
            name: None,
            dynamic: None,
            _type: None,
            content: None,
            priority: None,
        }
    }
}

/// Sets the snippet version.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Dynamic {
    #[serde(rename = "0")]
    Regular,
    #[serde(rename = "1")]
    Dynamic,
}

impl Default for Dynamic {
    fn default() -> Dynamic {
        Self::Regular
    }
}
/// The location in generated VCL where the snippet should be placed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "recv")]
    Recv,
    #[serde(rename = "hash")]
    Hash,
    #[serde(rename = "hit")]
    Hit,
    #[serde(rename = "miss")]
    Miss,
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fetch")]
    Fetch,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "deliver")]
    Deliver,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "none")]
    None,
}

impl Default for Type {
    fn default() -> Type {
        Self::Init
    }
}

