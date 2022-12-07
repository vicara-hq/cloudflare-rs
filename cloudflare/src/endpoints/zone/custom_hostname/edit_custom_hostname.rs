use serde::Serialize;

use crate::framework::endpoint::{EndpointSpec, Method};

use super::{CustomHostame, SslParams};

/// Delete Custom Hostname (and any issued SSL certificates)
/// https://api.cloudflare.com/#custom-hostname-for-a-zone-delete-custom-hostname-and-any-issued-ssl-certificates-
#[derive(Debug)]
pub struct EditCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
    pub params: EditCustomHostnameParams,
}

impl<'a> EndpointSpec<CustomHostame> for EditCustomHostname<'a> {
    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.identifier
        )
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct EditCustomHostnameParams {
    pub ssl: SslParams,
}
