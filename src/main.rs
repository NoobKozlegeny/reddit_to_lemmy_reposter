// Import libraries
use std::env;
use std::path::Path;
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
use reqwest::{Client, Response, multipart};
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
        "lemmy.basedcount.com".to_owned(),
        "main".to_owned(),
        "Owo 5".to_owned(),
        Some(Url::parse("https://i.pinimg.com/originals/f7/c0/e7/f7c0e76ef8fcf1c717364447e94a6702.jpg").unwrap()), // Some(Url::parse("https://hu.pinterest.com/pin/503769908335656123/").unwrap()),
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

async fn create_post_with_image(
    instance: String,
    community: String,
    name: String,
    url: String,
    body: Option<String>
) -> Result<String, Box<dyn std::error::Error>> {
    let auth = lemmy_auth(instance.clone()).await.unwrap();

    let form = multipart::Form::new()
        .text("name", name)
        .text("community_id", "1716")
        .text("body", body.unwrap())
        .text("auth", auth)
        .text("images[]", "https://i.pinimg.com/originals/f7/c0/e7/f7c0e76ef8fcf1c717364447e94a6702.jpg");

    let client = Client::new();
    let response = client
        .post(format!("https://{}/api/v3/post", instance))
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        Ok("Successful post with image!".to_string())
    } else {
        Err(format!("Unsuccessful post. Status code: {}", response.status()).into())
    }
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
