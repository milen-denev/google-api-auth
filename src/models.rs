use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct ServiceAccountKey {
    #[serde(rename = "type")]
    pub(crate) key_type: String,
    pub(crate) project_id: String,
    pub(crate) private_key_id: String,
    pub(crate) private_key: String,
    pub(crate) client_email: String,
    pub(crate) client_id: String,
    pub(crate) auth_uri: String,
    pub(crate) token_uri: String,
    pub(crate) auth_provider_x509_cert_url: String,
    pub(crate) client_x509_cert_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) iss: String,
    pub(crate) aud: String,
    pub(crate) scope: String,
    pub(crate) exp: usize,
    pub(crate) iat: usize,
}

/// Deserialized data from the json response of the OAUTH2 service endpoint.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
}

pub enum ServiceCredentialsInput {
    PathBuf(PathBuf),
    String(String)
}

impl From<PathBuf> for ServiceCredentialsInput {
    fn from(path_buf: PathBuf) -> Self {
        ServiceCredentialsInput::PathBuf(path_buf)
    }
}

impl From<String> for ServiceCredentialsInput {
    fn from(string: String) -> Self {
        ServiceCredentialsInput::String(string)
    }
}