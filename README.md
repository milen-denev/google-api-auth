# google-api-auth
## Dead simple API for generating access_token to query GCP's API

## Production Readiness 

This is already used in production in affiliated company for specific use-case. Use this in production at your own risk.

## Examples

### cargo.toml
```TOML
[dependencies]
serde = { version = "1.0", features = ["derive"] }
google-api-auth = "0.2.0"
```

### main.rs
```rust
fn main() {
    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.push("some-name-000000-000000000.json");

    let json_string = json!({
        "type": "service_account",
        "project_id": "some-name-000000",
        "private_key_id": "somerandomuuid000000000",
        "private_key": "-----BEGIN PRIVATE KEY-----\n SOME CERT DATA \n-----END PRIVATE KEY-----\n",
        "client_email": "some-name@some-account-000000.iam.gserviceaccount.com",
        "client_id": "000000000000000",
        "auth_uri": "https://accounts.google.com/o/oauth2/auth",
        "token_uri": "https://oauth2.googleapis.com/token",
        "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
        "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/some-account.iam.gserviceaccount.com",
        "universe_domain": "googleapis.com"
    }).to_string();

    //Create the handler.
    let handler = AuthenticationHandler::new(dir.into());

    //Handler using json `String`
    let handler_2 = AuthenticationHandler::new(json_string.into());

    //Get a token with scoped read / write access to GCP DNS API.
    let token = handler.get_access_token_model(
    vec!["https://www.googleapis.com/auth/ndev.clouddns.readwrite".into()]);

    println!("Access Token: {}", token.access_token);
}
```

## Affiliation with Google

This project isn't affiliated or maintained by Google, it's created by independent developer.