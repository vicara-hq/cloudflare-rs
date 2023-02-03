pub mod certificates;
pub mod custom_hostname;

use crate::framework::{
    endpoint::{Endpoint, Method},
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

impl Endpoint<Vec<Zone>, ListZonesParams> for ListZones {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "zones".to_string()
    }
    fn query(&self) -> Option<ListZonesParams> {
        Some(self.params.clone())
    }
}

/// Zone Details
/// <https://api.cloudflare.com/#zone-zone-details>
#[derive(Debug)]
pub struct ZoneDetails<'a> {
    pub identifier: &'a str,
}
impl<'a> Endpoint<Zone> for ZoneDetails<'a> {
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
impl<'a> Endpoint<Zone, (), CreateZoneParams<'a>> for CreateZone<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "zones".to_string()
    }

    fn body(&self) -> Option<CreateZoneParams<'a>> {
        Some(self.params.clone())
    }
}

/// Delete Zone
/// https://api.cloudflare.com/#zone-delete-zone
pub struct DeleteZone<'a> {
    pub identifier: &'a str,
}
impl<'a> Endpoint<DeleteZoneResponse, (), ()> for DeleteZone<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("zones/{}", self.identifier)
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteZoneResponse {
    pub id: String,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Owner {
    User {
        id: Option<String>,
        email: Option<String>,
    },
    Organization {
        id: String,
        name: String,
    },
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
impl ApiResult for DeleteZoneResponse {}
