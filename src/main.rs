use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;
use std::{io, thread};
use std::sync::Arc;
use tokio::sync::Mutex;

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

thread_local!(static AUTH_CODE: RefCell<String> = RefCell::new(String::new()));

async fn handle_request(
    req: hyper::Request<hyper::Body>,
    tx: tokio::sync::oneshot::Sender<()>,
) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
    // Handle the Reddit redirect  
    if let (&hyper::Method::GET, "/callback") = (req.method(), req.uri().path()) {
        // Get the query parameters from the URL  
        if let Some(query) = req.uri().query() {
            // Parse the parameters into a map  
            let params: HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();
            
            // Get the authorization code from the parameters  
            if let Some(code) = params.get("code") {
                // Code retrieval success
                println!("Authorization code: {}", code);

                // Send the shutdown signal
                let _ = tx.send(());
            } else {
                // No code present
                println!("No code found");
            }
        }
        // Respond with a '200 OK' status
        Ok(hyper::Response::new(hyper::Body::from("Received authorization code")))
    } else {
        // Respond with a '404 Not Found' for all other requests
        Ok(hyper::Response::builder()
            .status(hyper::StatusCode::NOT_FOUND) 
            .body(hyper::Body::from("404 Not Found"))
            .unwrap())
    }
}

#[tokio::main]
async fn main() { 
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    let make_service = hyper::service::make_service_fn(|_conn| {
        let tx = tx.clone();
        async {
            Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| handle_request(req, tx.clone())))
        }
    });
    
    let addr = ([127, 0, 0, 1], 8000).into();
    
    let server = hyper::Server::bind(&addr).serve(make_service);
    
    // Prepare some signal for when the server should start shutting down...
    let graceful = server
        .with_graceful_shutdown(async {
            rx.await.ok();
        });

    if reddit_authorize().await.is_ok() {
        println!("HIII");
    };
    //let _ = reddit_get_access_token();

    if let Err(e) = graceful.await {
        eprintln!("Server error: {}", e);
    }

    println!("YOOO");
}

async fn reddit_authorize() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "PhclXyWx_DimHWrojYdS8A";
    let client_secret = "uLUZ48K_Zo63Z-SAo7VA2m6AGU0WdQ";
    let random_string = "fqw8sdőfüó543!.@&xr";
    let redirect_uri = "http://localhost:8000/callback";
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

    Ok(())
}

async fn reddit_get_access_token() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "PhclXyWx_DimHWrojYdS8A";
    let client_secret = "uLUZ48K_Zo63Z-SAo7VA2m6AGU0WdQ";
    let random_string = "fqw8sdőfüó543!.@&xr";
    let redirect_uri = "http://localhost:8000/callback";
    let scope = "read";
    println!("KEEK");
    let mut auth_code: String = String::new();
    AUTH_CODE.with(|text| { text.borrow_mut().as_mut_str().clone_into(&mut auth_code); });

    // Select the authorization code from the link
    //https://www.reddit.com/user/UltimatePCAddict?state=Nl-oMq6xzv-Av5bHb9nBsw&code=CBf7BwGItbMSpAf4aqUtIirPFrt19A#_
    // let auth_code_url_vec: Vec<&str> = auth_code_url.split("code=").collect();
    // let auth_code = auth_code_url_vec[0].split("#_").next().unwrap().to_string();

    let client = BasicClient::new(
        ClientId::new(client_id.to_owned()),
        Some(ClientSecret::new(client_secret.to_owned())),
        AuthUrl::new("https://www.reddit.com/api/v1/authorize".to_owned())?,
        Some(TokenUrl::new("https://www.reddit.com/api/v1/access_token".to_string())?)
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(redirect_uri.to_owned())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Now you can trade it for an access token.
    let token_result = client
    .exchange_code(AuthorizationCode::new(auth_code))
    .add_extra_param("grant_type", "authorization_code")
    .add_extra_param("redirect_uri", redirect_uri)
    .set_pkce_verifier(pkce_verifier)
    .request_async(async_http_client)
    .await?;

    println!("{:?}", token_result);

    Ok(())
}