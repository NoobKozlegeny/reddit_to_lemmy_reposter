use std::collections::HashMap;

use reqwest::{self, header::{HeaderMap, HeaderValue, USER_AGENT}};
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

    let url = "https://www.reddit.com/api/v1/access_token/.json";
    let form = [
        ("grant_type", "password"),
        ("username", CLIENT_ID),
        ("password", CLIENT_SECRET),
    ];  

    let client = reqwest::Client::new();

    let request = client.post(url)
        .header(USER_AGENT, "linux:reposter:v0.0.1 (by u/UltimatePCAddict)")
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .form(&form);

    let response = request.send().await?;

    if response.status().is_success() {
        println!("Successful authorization!")
    }
    else {
        println!("{}", response.status().as_str())
    }


    Ok(())
}