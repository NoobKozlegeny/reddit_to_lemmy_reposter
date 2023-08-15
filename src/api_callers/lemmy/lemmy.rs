use std::{collections::HashMap, env, path::{Path, PathBuf}, fs::{File, OpenOptions}, io::{BufWriter, Write}};

use lazy_static::__Deref;
use lemmy_api_common::{community::GetCommunity, sensitive::Sensitive};
use serde_json::{json, Value};
use url::Url;

use crate::{
    statics::{CLIENT, HEADERS},
    structs::post::RedditPost,
};

pub async fn create_multiple_post(instance: String, community: String, posts: Vec<RedditPost>, path: PathBuf) -> String {
    for post in posts.clone() {
        let response = create_post(
        instance.clone(),
        community.clone(),
        post.id,
        post.title,
        Some(Url::parse(&post.url[..]).unwrap()), // Some(Url::parse("https://hu.pinterest.com/pin/503769908335656123/").unwrap()),
        Some(format!("Beep boop egy robot vagyok. \n
        Eredeti fostoló: {}", post.author).to_owned()),
        path.to_owned()
        ).await;
    }

    return format!("{} posts have been posted", posts.len().to_string());
}

pub async fn create_one_post(instance: String, community: String, post: Option<RedditPost>, path: PathBuf) -> Option<String> {
    let post_some = post?;
    let response = create_post(
        instance.clone(),
        community.clone(),
        post_some.id,
        post_some.title,
        Some(Url::parse(&post_some.url[..]).unwrap()), // Some(Url::parse("https://hu.pinterest.com/pin/503769908335656123/").unwrap()),
        Some(format!("Beep boop I'm a bot OwO \n
        Original poster: u/{} from Reddit \n", post_some.author).to_owned()),
        path
        ).await;

    if response.is_ok() {
        return Some("1 post have been posted".to_string());
    }
    else {
        return None;
    }   
}

async fn create_post(
    instance: String,
    community: String,
    id: String,
    name: String,
    url: Option<Url>,
    body: Option<String>,
    // honeypot: Option<String>,
    // nsfw: Option<bool>,
    // language_id: Option<LanguageId>
    path: PathBuf
) -> Result<String, Box<dyn std::error::Error>> {
    // Get community_id
    let community_id: u64 = get_community_id(community.clone(), instance.clone(), None)
        .await
        .unwrap();
    // Get auth code
    let auth = lemmy_auth(instance.clone()).await.unwrap();

    // Create CreatePost struct instance
    let params = json!({
        "name": name,
        "community_id": community_id,
        "url": url.unwrap(),
        "body": body.unwrap(),
        "auth": auth,
    });

    // Perform POST request
    let response = CLIENT
        .post(format!("https://{}/api/v3/post", instance))
        .headers(HEADERS.deref().clone())
        .json(&params)
        .send()
        .await?;

    // Return something based on success or fail
    if response.status().is_client_error() {
        return Err(format!(
            "Client Error when creating post. Status code: {}",
            response.status().as_str()
        ))?;
    } else if response.status().is_server_error() {
        return Err(format!(
            "Server Error when creating post. Status code: {}",
            response.status().as_str()
        ))?;
    } else {
        // Write the id to file
        write_to_file(&path, id);
        return Ok("Successful post!".to_string());
    }
}

pub async fn lemmy_auth(instance: String) -> Result<String, Box<dyn std::error::Error>> {
    // Get credentials from environemnt (export LEMMY_AUTH_PASSWORD=secretpassword)
    let username_or_email = env::var("NAME_OR_EMAIL").expect("NAME_OR_EMAIL must be set");
    let password = env::var("LEMMY_AUTH_PASSWORD").expect("LEMMY_AUTH_PASSWORD must be set");

    let mut params = HashMap::new();
    params.insert("username_or_email", &username_or_email);
    params.insert("password", &password);

    let response = CLIENT
        .post(format!("https://{}/api/v3/user/login", instance))
        .headers(HEADERS.deref().clone())
        .json(&params)
        .send()
        .await;

    let response_unwrapped = response?;
    if response_unwrapped.status().is_success() {
        let value_json: Value = serde_json::from_str(&response_unwrapped.text().await.ok().unwrap()[..])?;
        let mut jwt = value_json["jwt"].to_string();
        jwt.remove(0);
        jwt.remove(jwt.len() - 1);
        return Ok(jwt);
    }
    else {
        return Err("Failed authorization!".into());
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
        .headers(HEADERS.deref().clone())
        .query(&params)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the response body as JSON
        let search_json: Value = serde_json::from_str(&response.text().await.unwrap())?;
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

fn write_to_file(path: &Path, id: String) {
    println!("Path: {:?}", path);
    println!("Current Path: {:?}", env::current_dir());
    // Open file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    let _ = file.write(b"\n");
    let write_result = file.write_all(id.as_bytes());

    if write_result.is_err() {
        println!("Error with writing {:#?}", write_result.err());
    }
}
