use std::{env, collections::HashMap};

use hyper::{HeaderMap, http::HeaderValue};
use lemmy_api_common::{sensitive::Sensitive, community::GetCommunity};
use serde_json::{json, Value};
use url::Url;

use crate::statics::CLIENT;

pub async fn create_post(
    instance: String,
    community: String,
    name: String,
    url: Option<Url>,
    body: Option<String>,
    // honeypot: Option<String>,
    // nsfw: Option<bool>,
    // language_id: Option<LanguageId>
) -> Result<String, Box<dyn std::error::Error>> {
    // Get community_id
    let community_id: u64 =
        get_community_id("main".to_string(), instance.clone(), None)
            .await
            .unwrap();
    // Get auth code
    let auth = lemmy_auth(instance.clone())
        .await
        .unwrap();
    println!("{}", auth);

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        "User-Agent",
        HeaderValue::from_str("reddit_to_lemmy_reposter (by u/PrivateNoob@sopuli.xyz)").unwrap(),
    );
    // Create CreatePost struct instance
    let params = json!({
        "name": name,
        "community_id": community_id,
        "url": url.unwrap(),
        "body": body.unwrap(),
        "auth": auth,
    });
    // println!("{:#?}", params);
    // Perform POST request
    let response = CLIENT
        .post(format!("https://{}/api/v3/post", instance))
        .headers(headers)
        .json(&params)
        .send()
        .await;
    let kek = response.unwrap().text().await.unwrap();

    return Ok("Successful post!".to_string());
}

pub async fn lemmy_auth(instance: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        "User-Agent",
        HeaderValue::from_str("reddit_to_lemmy_reposter (by u/PrivateNoob@sopuli.xyz)").unwrap(),
    );

    // Get credentials from environemnt (export LEMMY_AUTH_PASSWORD=secretpassword)
    let username_or_email = env::var("NAME_OR_EMAIL").expect("NAME_OR_EMAIL must be set");
    let password = env::var("LEMMY_AUTH_PASSWORD").expect("LEMMY_AUTH_PASSWORD must be set");

    let mut params = HashMap::new();
    params.insert("username_or_email", &username_or_email);
    params.insert("password", &password);

    let response = CLIENT
        .post(format!("https://{}/api/v3/user/login", instance))
        .headers(headers)
        .json(&params)
        .send()
        .await;

    match response {
        Ok(value) => {
            let value_json: Value = serde_json::from_str(&value.text().await.ok().unwrap()[..])?;
            let mut jwt = value_json["jwt"].to_string();
            jwt.remove(0);
            jwt.remove(jwt.len() - 1);
            return Ok(jwt);
        }
        Err(err) => return Err(err)?,
    }
}

pub async fn get_community_id(
    name: String,
    instance: String,
    auth: Option<Sensitive<String>>,
) -> Result<u64, Box<dyn std::error::Error>> {
    // Construct the search parameters
    let params = json!({
        "q": name.clone(),
        "type_": "Communities",
        "sort": "Hot",
        "page": 1,
        "limit": 1
    });

    // Send a GET request to the search endpoint
    let response = CLIENT
        .get(format!("https://{}/api/v3/search", instance))
        .query(&params)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the response body as JSON
        let search_json: Value = serde_json::from_str(&response.text().await.unwrap())?;
        println!("{:#?}",search_json);
        // Extract the community_id from the response
        let community_id = search_json["communities"][0]["community"]["id"]
            .as_u64()
            .unwrap();

        // Return the community_id
        return Ok(community_id);
    } else {
        // If the request was not successful, return an error
        return Err(format!(
            "Failed community request. Status code: {}",
            response.status().as_str()
        )
        .to_owned())?;
    }
}