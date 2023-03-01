use serde::{Deserialize, Serialize};

use crate::framework::{
    endpoint::{Endpoint, Method},
    response::ApiResult,
};

/// Revoke Certificate
/// https://developers.cloudflare.com/api/operations/origin-ca-revoke-certificate
#[derive(Debug)]
pub struct RevokeCertificate<'a> {
    pub identifier: &'a str,
}

impl<'a> Endpoint<RevokedCertificateID, (), ()> for RevokeCertificate<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!("certificates/{}", self.identifier)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RevokedCertificateID {
    pub id: String,
}

impl ApiResult for RevokedCertificateID {}
