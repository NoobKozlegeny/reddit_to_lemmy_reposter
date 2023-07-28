// Import libraries
use std::env;
use std::time::Duration;
use std::{collections::HashMap, error::Error};

// Import own modules / Create module tree
use crate::{api_callers::reddit::reddit_caller::reddit_get_posts, structs::post::Post};
pub mod api_callers;
pub mod structs;

use once_cell::sync::Lazy;
use hyper::{http::HeaderValue, HeaderMap};
use lemmy_api_common::lemmy_db_schema::newtypes::LanguageId;
use lemmy_api_common::person::Login;
use lemmy_api_common::{
    lemmy_db_schema::newtypes::CommunityId, post::CreatePost, sensitive::Sensitive,
};
use reqwest::{Client, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::{Value, json};

use lemmy_api_common::community::{GetCommunity, GetCommunityResponse};
use url::Url;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    return Client::new();
});

#[tokio::main]
async fn main() {
    // Get posts from subreddit
    let posts: Result<Vec<Post>, Box<dyn Error>> = reddit_get_posts("fosttalicska", 3).await;
    match &posts {
        Ok(value) => println!("Successfully fetched posts!"),
        Err(err) => println!("{}", err),
    }

    // Print the number of posts gathered
    println!("{}", posts.as_ref().unwrap().iter().count());

    // Create a post to Lemmy
    let post_response = create_post(
        "voyager.lemmy.ml".to_owned(),
        "main".to_owned(),
        "OwO".to_owned(),
        Some(Url::parse("https://hu.pinterest.com/pin/503769908335656123/").unwrap()),
        Some("sniff sniff i-is that a BOY I smell? sniff sniff mmm yes I smell it! BOYSMELL!!!! I smell a boy! W-What is a boy doing here?!?! omygosh what am I gonna do?!?! THERE'S A BOY HERE! I'M FREAKING OUT SO MUCH!!!! calm down calm down and take a nice, deep breathe.... sniff sniff it smells so good! I love boysmell so much!!!! It makes me feel so amazing. I'm getting tingles all over from the delicious boyscent! It's driving me boyCRAZY!!!!!!".to_owned())
    ).await;
    println!("{}", post_response.unwrap());
    print!("...");
}

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
    let community_id: CommunityId = CommunityId {
        0: i32::try_from(1716)?,
    };
    // Get auth code
    // let auth = lemmy_auth("lemmy.basedcount.com".to_string())
    //     .await
    //     .unwrap();
    // println!("{}", auth.len());

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
    let params_json = json!({
        "name": "OwO",
        "community_id": 1716,
        "url": "https://hu.pinterest.com/pin/503769908335656123/",
        "body": "Hewwo",
        "auth": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjI1LCJpc3MiOiJsZW1teS5iYXNlZGNvdW50LmNvbSIsImlhdCI6MTY5MDU3MjQ5MH0.12lWqboT2QNht3v4lvVOBPsd3Z31j8Th11khMNoCtOU",
        "honeypot": "",
        "language_id": 0,
        "nsfw": false
    });
    let params: CreatePost = CreatePost {
        name: "OwO".to_owned(),
        community_id,
        url,
        body,
        // auth,
        ..Default::default()
    };
    // Perform POST request
    let response = CLIENT
        .post(format!("https://lemmy.basedcount.com/api/v3/post"))
        .headers(headers)
        .json(&params_json)
        .send()
        .await;

    let kek = response.unwrap().text().await.unwrap();

    return Ok("Successful post!".to_string());
}

pub async fn lemmy_auth(instance: String) -> Result<Sensitive<String>, Box<dyn std::error::Error>> {
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
        .post(format!("https://lemmy.basedcount.com/api/v3/user/login"))
        .headers(headers)
        .json(&params)
        .send()
        .await;

    match response {
        Ok(value) => {
            let value_json: Value = serde_json::from_str(&value.text().await.ok().unwrap()[..])?;
            return Ok(Sensitive::from(value_json["jwt"].to_string()));
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
