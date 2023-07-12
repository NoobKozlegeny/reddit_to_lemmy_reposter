use std::collections::HashMap;

use reqwest::{self, header::{HeaderMap, HeaderValue}};
// use serde::Deserialize;
use tokio;
use serde_json::json;

#[tokio::main]
async fn main() {
    let _ = reddit_authorize().await;
}

async fn reddit_authorize() -> Result<(), reqwest::Error> {
    let CLIENT_ID = "PhclXyWx_DimHWrojYdS8A";
    let CLIENT_SECRET = "uLUZ48K_Zo63Z-SAo7VA2m6AGU0WdQ";

    let random_string = "fsd8jü32fd##!!";
    // let request_url = format!(
    //     "https://www.reddit.com/api/v1/authorize?client_id={CLIENT_ID}&response_type={TYPE}&state={RANDOM_STRING}&redirect_uri={URI}&duration={DURATION}&scope={SCOPE_STRING}",
    //     CLIENT_ID = "PhclXyWx_DimHWrojYdS8A",
    //     TYPE = "code",
    //     RANDOM_STRING = random_string,
    //     URI = "https://discordapp.com/users/334419819627675648",
    //     DURATION = "permanent",
    //     SCOPE_STRING = "read"
    // );

    let client = reqwest::Client::new();

    let mut body_map = HashMap::new();
    body_map.insert("grant_type".to_string(), "password".to_string());
    body_map.insert("username".to_string(), "UltimatePCAddict".to_string());
    body_map.insert("password".to_string(), "Azolos10".to_string());

    let body_json = json!(body_map).to_string();

    let mut header_map = HeaderMap::new();
    header_map.insert("User-Agent", HeaderValue::from_static("Reposter/0.0.0 (by u/UltimatePCAddict)"));
    header_map.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));

    let response = client.post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .body(body_json)
        .headers(header_map)
        .send()
        .await?;   

    if response.status().is_success() {
        let xd = response.text().await?;
        println!("{}", xd)
    }
    else {
        println!("{}", response.status().as_str())
    }


    Ok(())
}