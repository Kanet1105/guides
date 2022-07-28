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
use url::Url;

pub fn get_login_url() -> Url {
    let client = BasicClient::new(
        ClientId::new("516738924199-iapd4m4egftj36tqbrlrokhebfst2oup.apps.googleusercontent.com".to_string()),
        None,
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).unwrap()),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://127.0.0.1:5000/auth".to_string()).unwrap(),
    );

    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    authorize_url
}