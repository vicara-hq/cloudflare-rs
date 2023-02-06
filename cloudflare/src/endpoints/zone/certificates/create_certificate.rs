use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::ApiResult,
};

/// Create new origin CA certificate
/// https://developers.cloudflare.com/api/operations/origin-ca-create-certificate
#[derive(Debug)]
pub struct CreateCertifcate {
    pub body: CreateCertifcateBody,
}

impl EndpointSpec<CreateCertifcateResponse> for CreateCertifcate {
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "certificates".to_string()
    }
    #[inline]
    fn body(&self) -> Option<String> {
        let body = serde_json::to_string(&self.body).unwrap();
        Some(body)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateCertifcateBody {
    pub csr: String,
    pub hostnames: Vec<String>,
    pub request_type: CertificateRequestType,
    pub requested_validity: CertificateRequestedValidity,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CertificateRequestType {
    OriginRsa,
    OriginEcc,
    KeylessCertificate,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CertificateRequestedValidity {
    Days_7 = 7,
    Days_30 = 30,
    Days_90 = 90,
    Days_365 = 365,
    Days_730 = 730,
    Days_1095 = 1095,
    Days_5475 = 5475,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateCertifcateResponse {
    pub id: String,
    pub certificate: String,
}

impl ApiResult for CreateCertifcateResponse {}
