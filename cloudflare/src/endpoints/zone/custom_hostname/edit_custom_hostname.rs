use crate::framework::endpoint::{Endpoint, Method};

use super::{CustomHostame, SslParams};

/// Delete Custom Hostname (and any issued SSL certificates)
/// https://api.cloudflare.com/#custom-hostname-for-a-zone-delete-custom-hostname-and-any-issued-ssl-certificates-
#[derive(Debug)]
pub struct EditCustomHostname<'a> {
    pub zone_identifier: &'a str,
    pub identifier: &'a str,
    pub params: EditCustomHostnameParams,
}

impl<'a> Endpoint<CustomHostame, (), EditCustomHostnameParams> for EditCustomHostname<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_identifier, self.identifier
        )
    }
    fn body(&self) -> Option<EditCustomHostnameParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct EditCustomHostnameParams {
    pub ssl: SslParams,
}
