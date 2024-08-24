use models::{AuthResponse, Claims, ServiceAccountKey, ServiceCredentialsInput};
use reqwest::blocking::Client;
use std::{fs, time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

mod models;

fn create_jwt(key: &ServiceAccountKey, scopes: Vec<String>) -> String {
    let iat = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    let exp = iat + 3600;

    let scopes = {
        let count = scopes.iter().count();
        if count > 1 {
            let mut scopes_string = scopes.iter().take(count - 1).map(|x| x.to_string() + ",").collect::<String>();
            scopes_string.push_str(&scopes[count - 1]);
            scopes_string
        }
        else {
            let scope = scopes[0].clone();
            drop(scopes);
            scope
        }
    };

    let claims = Claims {
        iss: key.client_email.clone(),
        scope: scopes,
        aud: key.token_uri.clone(),
        exp,
        iat,
    };

    let encoding_key = EncodingKey::from_rsa_pem(key.private_key.as_bytes()).expect("Invalid RSA Key");
    encode(&Header::new(Algorithm::RS256), &claims, &encoding_key).expect("JWT Encoding failed")
}

fn get_access_token(key: &ServiceAccountKey, scopes: Vec<String>) -> AuthResponse {
    let jwt = create_jwt(key, scopes);

    let params = format!("grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={}",jwt);

    let client = Client::new();
    
    let res = client.post(format!("https://oauth2.googleapis.com/token"))
        .header("content-type", "application/x-www-form-urlencoded")
        .body(params)
        .send()
        .expect("Failed to get access token");

    let content = res.text();

    let token_response: AuthResponse = serde_json::from_str(
        &content.expect("Failed to get text out of response.")
    ).expect("Failed to parse token response");
    token_response
}

/// Authentication handler for storing json credentials and requesting new access_token then necessary.
/// 
/// More details: https://developers.google.com/identity/protocols/oauth2/service-account
/// 
/// ```rust
/// //Example if json credentials are stored at the same directory where the program is contained.
/// let mut dir = env::current_exe().unwrap();
/// dir.pop();
/// dir.push("some-name-431008-92e3a679a62f.json");
/// 
/// let json_string = json!({
///     "type": "service_account",
///     "project_id": "some-name-0000000",
///     "private_key_id": "somerandomuuid000000000",
///     "private_key": "-----BEGIN PRIVATE KEY-----\n SOME CERT DATA \n-----END PRIVATE KEY-----\n",
///     "client_email": "some-name@some-account-0000000.iam.gserviceaccount.com",
///     "client_id": "000000000000000",
///     "auth_uri": "https://accounts.google.com/o/oauth2/auth",
///     "token_uri": "https://oauth2.googleapis.com/token",
///     "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
///     "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/some-account.iam.gserviceaccount.com",
///     "universe_domain": "googleapis.com"
/// }).to_string();
/// 
/// //Create the handler.
/// let handler = AuthenticationHandler::new(dir.into());
/// 
/// //Handler using json `String`
/// let handler_2 = AuthenticationHandler::new(json_string.into());
/// 
/// //Get a token with scoped read / write access to GCP DNS API.
/// let token = handler.get_access_token_model(
///     vec!["https://www.googleapis.com/auth/ndev.clouddns.readwrite".into()]);
/// 
/// println!("Access Token: {}", token.access_token);
/// ```
pub struct AuthenticationHandler {
    service_credentials: ServiceAccountKey
}

impl AuthenticationHandler {
    /// Creates new `AuthenticationHandler`. Requires a `PathBuf` or json `String` containing the service account credentials (key).
    pub fn new(creds: ServiceCredentialsInput) -> AuthenticationHandler {
        match creds {
            ServiceCredentialsInput::PathBuf(creds) => {
                let key_data = fs::read_to_string(creds)
                    .expect("Failed to read service account key file");
                let service_account_key: ServiceAccountKey = serde_json::from_str(&key_data)
                    .expect("Failed to parse service account key");
                AuthenticationHandler {
                    service_credentials: service_account_key
                }
            },
            ServiceCredentialsInput::String(creds) => {
                let service_account_key: ServiceAccountKey = serde_json::from_str(&creds)
                    .expect("Failed to parse service account key");
                AuthenticationHandler {
                    service_credentials: service_account_key
                }
            }
        }
    }

    /// Creates new `access_token` with specific access. Please for complete scopes list refer to: `https://developers.google.com/identity/protocols/oauth2/scopes`. Make sure to give the respective access /role to the service account. 
    pub fn get_access_token_model(&self, scopes: Vec<String>) -> AuthResponse {
        let token = get_access_token(&self.service_credentials, scopes);
        token
    }
}