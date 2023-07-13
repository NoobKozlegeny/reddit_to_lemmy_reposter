use anyhow;
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
    TokenUrl, ResponseType
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use url::Url;

#[tokio::main]
async fn main() {
    let _ = reddit_authorize().await;
}

async fn reddit_authorize() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "PhclXyWx_DimHWrojYdS8A";
    let client_secret = "uLUZ48K_Zo63Z-SAo7VA2m6AGU0WdQ";
    let random_string = "fqw8sdőfüó543!.@&xr";
    let redirect_uri = "https://www.reddit.com/user/UltimatePCAddict";
    let scope = "read";

    let auth_url = format!(
        "https://www.reddit.com/api/v1/authorize?client_id={}&response_type=code&state={}&redirect_uri={}&duration=permanent&scope={}",
        client_id, random_string, redirect_uri, scope
    );

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(client_id.to_owned()),
        Some(ClientSecret::new(client_secret.to_owned())),
        AuthUrl::new("https://www.reddit.com/api/v1/authorize".to_owned())?,
        Some(TokenUrl::new("https://www.reddit.com/api/v1/access_token".to_string())?)
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(redirect_uri.to_owned())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    let kek = "XD";

    Ok(())
}