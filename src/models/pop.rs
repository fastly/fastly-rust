/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Pop {
    /// the three-letter code for the [POP](https://developer.fastly.com/learning/concepts/pop/)
    #[serde(rename = "code")]
    pub code: String,
    /// the name of the POP
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "group")]
    pub group: String,
    #[serde(rename = "region")]
    pub region: Region,
    /// the region used for stats reporting
    #[serde(rename = "stats_region")]
    pub stats_region: StatsRegion,
    /// the region used for billing
    #[serde(rename = "billing_region")]
    pub billing_region: BillingRegion,
    #[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Box<crate::models::PopCoordinates>>,
    /// the name of the [shield code](https://developer.fastly.com/learning/concepts/shielding/#choosing-a-shield-location) if this POP is suitable for shielding
    #[serde(rename = "shield", skip_serializing_if = "Option::is_none")]
    pub shield: Option<String>,
}

impl Pop {
    pub fn new(code: String, name: String, group: String, region: Region, stats_region: StatsRegion, billing_region: BillingRegion) -> Pop {
        Pop {
            code,
            name,
            group,
            region,
            stats_region,
            billing_region,
            coordinates: None,
            shield: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "APAC")]
    APAC,
    #[serde(rename = "Asia")]
    Asia,
    #[serde(rename = "AF-West")]
    AFWest,
    #[serde(rename = "EU-Central")]
    EUCentral,
    #[serde(rename = "EU-East")]
    EUEast,
    #[serde(rename = "EU-West")]
    EUWest,
    #[serde(rename = "Middle-East")]
    MiddleEast,
    #[serde(rename = "North-America")]
    NorthAmerica,
    #[serde(rename = "SA-South")]
    SASouth,
    #[serde(rename = "SA-East")]
    SAEast,
    #[serde(rename = "SA-West")]
    SAWest,
    #[serde(rename = "SA-North")]
    SANorth,
    #[serde(rename = "South-Africa")]
    SouthAfrica,
    #[serde(rename = "South-America")]
    SouthAmerica,
    #[serde(rename = "US-Central")]
    USCentral,
    #[serde(rename = "US-East")]
    USEast,
    #[serde(rename = "US-West")]
    USWest,
    #[serde(rename = "Asia-South")]
    AsiaSouth,
}

impl Default for Region {
    fn default() -> Region {
        Self::APAC
    }
}
/// the region used for stats reporting
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatsRegion {
    #[serde(rename = "southamerica_std")]
    SouthamericaStd,
    #[serde(rename = "africa_std")]
    AfricaStd,
    #[serde(rename = "anzac")]
    Anzac,
    #[serde(rename = "asia")]
    Asia,
    #[serde(rename = "europe")]
    Europe,
    #[serde(rename = "usa")]
    Usa,
    #[serde(rename = "asia_india")]
    AsiaIndia,
    #[serde(rename = "asia_southkorea")]
    AsiaSouthkorea,
}

impl Default for StatsRegion {
    fn default() -> StatsRegion {
        Self::SouthamericaStd
    }
}
/// the region used for billing
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingRegion {
    #[serde(rename = "Africa")]
    Africa,
    #[serde(rename = "Australia")]
    Australia,
    #[serde(rename = "Asia")]
    Asia,
    #[serde(rename = "Europe")]
    Europe,
    #[serde(rename = "India")]
    India,
    #[serde(rename = "North America")]
    NorthAmerica,
    #[serde(rename = "South Korea")]
    SouthKorea,
    #[serde(rename = "South America")]
    SouthAmerica,
}

impl Default for BillingRegion {
    fn default() -> BillingRegion {
        Self::Africa
    }
}

