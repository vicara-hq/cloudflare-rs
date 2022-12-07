use serde::Serialize;

use crate::framework::endpoint::{EndpointSpec, Method};

use super::{CustomHostame, SslParams};

/// Add a new custom hostname and request that an SSL certificate be issued for it
/// https://api.cloudflare.com/#custom-hostname-for-a-zone-create-custom-hostname
#[derive(Debug)]
pub struct CreateCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub params: CreateCustomHostnameParams,
}

impl<'a> EndpointSpec<CustomHostame> for CreateCustomHostname<'a> {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/custom_hostnames", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateCustomHostnameParams {
    pub hostname: String,
    pub ssl: SslParams,
}
