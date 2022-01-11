use crate::framework::endpoint::serialize_query;
use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::ApiResult,
};
use crate::framework::{OrderDirection, SearchMatch};
use serde::{Deserialize, Serialize};

/// List Zones
/// List, search, sort, and filter your zones
/// <https://api.cloudflare.com/#zone-list-zones>
#[derive(Debug)]
pub struct ListZones {
    pub params: ListZonesParams,
}

impl EndpointSpec<Vec<Zone>> for ListZones {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "zones".to_string()
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

/// Zone Details
/// <https://api.cloudflare.com/#zone-zone-details>
#[derive(Debug)]
pub struct ZoneDetails<'a> {
    pub identifier: &'a str,
}
impl<'a> EndpointSpec<Zone> for ZoneDetails<'a> {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("zones/{}", self.identifier)
    }
}

/// Add Zone
/// <https://api.cloudflare.com/#zone-create-zone>
pub struct CreateZone<'a> {
    pub params: CreateZoneParams<'a>,
}
impl<'a> EndpointSpec<Zone> for CreateZone<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "zones".to_string()
    }

    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

/// Delete Zone
/// https://api.cloudflare.com/#zone-delete-zone
pub struct DeleteZone<'a> {
    pub identifier: &'a str,
}
impl<'a> EndpointSpec<()> for DeleteZone<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("zones/{}", self.identifier)
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct AccountId<'a> {
    pub id: &'a str,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct CreateZoneParams<'a> {
    pub name: &'a str,
    pub account: AccountId<'a>,
    pub jump_start: Option<bool>,
    #[serde(rename = "type")]
    pub zone_type: Option<Type>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListZonesParams {
    pub name: Option<String>,
    pub status: Option<Status>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<ListZonesOrder>,
    pub direction: Option<OrderDirection>,
    #[serde(rename = "match")]
    pub search_match: Option<SearchMatch>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListZonesOrder {
    Name,
    Status,
    Email,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "status", rename_all = "lowercase")]
pub enum Status {
    Active,
    Pending,
    Initializing,
    Moved,
    Deleted,
    Deactivated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Full,
    Partial,
}

/// A Zone is a domain name along with its subdomains and other identities
/// <https://api.cloudflare.com/#zone-properties>
#[derive(Deserialize, Debug)]
pub struct Zone {
    /// Zone identifier tag
    pub id: String,
    /// The domain name
    pub name: String,
    /// Status of the zone
    pub status: Status,
}

// TODO: This should probably be a derive macro
impl ApiResult for Zone {}
impl ApiResult for Vec<Zone> {}
