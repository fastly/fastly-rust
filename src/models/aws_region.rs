/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// AwsRegion : A named set of [AWS resources](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints) that's in the same geographical area.

/// A named set of [AWS resources](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints) that's in the same geographical area.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AwsRegion {
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "us-east-2")]
    UsEast2,
    #[serde(rename = "us-west-1")]
    UsWest1,
    #[serde(rename = "us-west-2")]
    UsWest2,
    #[serde(rename = "af-south-1")]
    AfSouth1,
    #[serde(rename = "ap-east-1")]
    ApEast1,
    #[serde(rename = "ap-south-1")]
    ApSouth1,
    #[serde(rename = "ap-northeast-3")]
    ApNortheast3,
    #[serde(rename = "ap-northeast-2")]
    ApNortheast2,
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,
    #[serde(rename = "ap-southeast-2")]
    ApSoutheast2,
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,
    #[serde(rename = "ca-central-1")]
    CaCentral1,
    #[serde(rename = "cn-north-1")]
    CnNorth1,
    #[serde(rename = "cn-northwest-1")]
    CnNorthwest1,
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    #[serde(rename = "eu-west-1")]
    EuWest1,
    #[serde(rename = "eu-west-2")]
    EuWest2,
    #[serde(rename = "eu-south-1")]
    EuSouth1,
    #[serde(rename = "eu-west-3")]
    EuWest3,
    #[serde(rename = "eu-north-1")]
    EuNorth1,
    #[serde(rename = "me-south-1")]
    MeSouth1,
    #[serde(rename = "sa-east-1")]
    SaEast1,

}

impl ToString for AwsRegion {
    fn to_string(&self) -> String {
        match self {
            Self::UsEast1 => String::from("us-east-1"),
            Self::UsEast2 => String::from("us-east-2"),
            Self::UsWest1 => String::from("us-west-1"),
            Self::UsWest2 => String::from("us-west-2"),
            Self::AfSouth1 => String::from("af-south-1"),
            Self::ApEast1 => String::from("ap-east-1"),
            Self::ApSouth1 => String::from("ap-south-1"),
            Self::ApNortheast3 => String::from("ap-northeast-3"),
            Self::ApNortheast2 => String::from("ap-northeast-2"),
            Self::ApSoutheast1 => String::from("ap-southeast-1"),
            Self::ApSoutheast2 => String::from("ap-southeast-2"),
            Self::ApNortheast1 => String::from("ap-northeast-1"),
            Self::CaCentral1 => String::from("ca-central-1"),
            Self::CnNorth1 => String::from("cn-north-1"),
            Self::CnNorthwest1 => String::from("cn-northwest-1"),
            Self::EuCentral1 => String::from("eu-central-1"),
            Self::EuWest1 => String::from("eu-west-1"),
            Self::EuWest2 => String::from("eu-west-2"),
            Self::EuSouth1 => String::from("eu-south-1"),
            Self::EuWest3 => String::from("eu-west-3"),
            Self::EuNorth1 => String::from("eu-north-1"),
            Self::MeSouth1 => String::from("me-south-1"),
            Self::SaEast1 => String::from("sa-east-1"),
        }
    }
}

impl Default for AwsRegion {
    fn default() -> AwsRegion {
        Self::UsEast1
    }
}




