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
    // let community_id_u64: u64 =
    //     get_community_id("lemmy.basedcount.com".to_string(), instance.clone(), None)
    //         .await
    //         .unwrap();
    // let community_id: CommunityId = CommunityId {
    //     0: i32::try_from(1716)?,
    // };
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
        "community_id": 1716,
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
    let params = GetCommunity {
        name: Some(name),
        auth,
        ..Default::default()
    };

    let response = CLIENT
        .get(format!("https://{}/api/v3/community", instance))
        .query(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let community_json: Value = serde_json::from_str(&response.text().await.unwrap())?;
        println!("{:#?}", community_json);
        let community_id = community_json["community_view"]["community"]["id"]
            .as_u64()
            .unwrap();
        return Ok(community_id);
    } else {
        return Err(format!(
            "Unsuccesful community request. Status code: {}",
            response.status().as_str()
        )
        .to_owned())?;
    }
}