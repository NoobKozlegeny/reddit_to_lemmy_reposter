// Import libraries
use std::env;
use std::{error::Error, collections::HashMap};

// Import own modules / Create module tree
use crate::{api_callers::reddit::reddit_caller::reddit_get_posts, structs::post::Post};
pub mod api_callers;
pub mod structs;

use hyper::{HeaderMap, http::HeaderValue};
use lemmy_api_common::{post::CreatePost, sensitive::Sensitive, lemmy_db_schema::newtypes::CommunityId};
use reqwest::{Client, Response};
use lemmy_api_common::person::Login;
use serde_json::Value;

use lemmy_api_common::community::{GetCommunity, GetCommunityResponse};
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
    // let post: CreatePost = CreatePost {
    //     name: "OwO".to_string(),
    //     community_id: CommunityId(),
    //     url: (),
    //     body: (),
    //     honeypot: (),
    //     nsfw: (),
    //     language_id: (),
    //     auth: (),
    // };
    // create_post("sopuli.xyz".to_string(), "fosttalicska".to_string(), post);
    
    // let jwt = lemmy_auth("lemmy.basedcount.com".to_string()).await.unwrap();
    // println!("{}", jwt);

    let community_id = get_community_id("fosttalicska".to_owned(), "sopuli.xyz".to_owned(), None)
        .await
        .unwrap();
    println!("{:#?}", community_id);
}

pub async fn create_post(
    instance: String,
    community: String,
    post: CreatePost,
) -> Result<String, Box<dyn std::error::Error>> {
    // let kek: Sensitive<String> = "^9vBHpNbP9L8Q8*tC9%vW^Cs";
    let params = post;

    return Ok("Successful post!".to_string());
}

pub async fn lemmy_auth(instance: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    headers.insert("User-Agent", HeaderValue::from_str("reddit_to_lemmy_reposter (by u/PrivateNoob@sopuli.xyz)").unwrap());
    
    // Get credentials from environemnt (export LEMMY_AUTH_PASSWORD=secretpassword)
    let username_or_email = env::var("NAME_OR_EMAIL").expect("NAME_OR_EMAIL must be set");
    let password = env::var("LEMMY_AUTH_PASSWORD").expect("LEMMY_AUTH_PASSWORD must be set");

    let mut params = HashMap::new();
    params.insert("username_or_email", &username_or_email);
    params.insert("password", &password);

    let client = Client::new();
    let response = client.post(format!("https://{}/api/v3/user/login", instance))
    .headers(headers)
    .json(&params)
    .send()
    .await;

    match response {
        Ok(value) => {
            let value_json: Value = serde_json::from_str(&value.text().await.ok().unwrap()[..])?;
            return Ok(value_json["jwt"].to_string());
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

    let client = Client::new();
    let response = client
        .get(format!("https://{}/api/v3/community", instance))
        .query(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let community_json: Value = serde_json::from_str(&response.text().await.unwrap())?;
        let community_id = community_json["community_view"]["community"]["id"].as_u64().unwrap();
        return Ok(community_id);
    }
    else {
        return Err(format!("Unsuccesful community request. Status code: {}", response.status().as_str()).to_owned())?;
    }
}
