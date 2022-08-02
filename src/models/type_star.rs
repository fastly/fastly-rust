/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// TypeStar : Resource type

/// Resource type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeStar {
    #[serde(rename = "star")]
    Star,

}

impl ToString for TypeStar {
    fn to_string(&self) -> String {
        match self {
            Self::Star => String::from("star"),
        }
    }
}

impl Default for TypeStar {
    fn default() -> TypeStar {
        Self::Star
    }
}




