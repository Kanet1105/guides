#[allow(unused)]
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};

use oauth2::basic::BasicClient;
// use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use serde::Deserialize;

pub fn get_oauth_client() -> Result<BasicClient, std::io::Error> {
    let oauth_client = BasicClient::new(
        ClientId::new("516738924199-iapd4m4egftj36tqbrlrokhebfst2oup.apps.googleusercontent.com".to_string()),
        None,
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).unwrap()),
    ).set_redirect_uri(
        RedirectUrl::new("http://127.0.0.1:8080/auth".to_string()).unwrap(),
    );

    Ok(oauth_client)
}

pub fn get_login_url(client: &BasicClient) -> Url {
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    authorize_url
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub state: String,
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub prompt: String,
}

pub fn exchange_token(client: &BasicClient) {
}