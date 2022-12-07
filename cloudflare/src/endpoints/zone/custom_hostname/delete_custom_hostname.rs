use serde::{Deserialize, Serialize};

use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::ApiResult,
};

/// Delete Custom Hostname (and any issued SSL certificates)
/// https://api.cloudflare.com/#custom-hostname-for-a-zone-delete-custom-hostname-and-any-issued-ssl-certificates-
#[derive(Debug)]
pub struct DeleteCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
}

impl<'a> EndpointSpec<CustomHostameDeleteID> for DeleteCustomHostname<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.identifier
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CustomHostameDeleteID {
    pub id: String,
}
impl ApiResult for CustomHostameDeleteID {}
