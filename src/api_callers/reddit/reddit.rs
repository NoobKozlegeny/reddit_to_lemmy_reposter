use std::error::Error;
use serde_json::Value;

use crate::structs::post::RedditPost;

pub async fn reddit_get_posts(
    subreddit: &str,
    start_idx: usize,
) -> Result<Vec<RedditPost>, Box<dyn Error>> {
    // ...
    let url: String = format!("https://www.reddit.com/r/{}/hot.json", subreddit);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(
            reqwest::header::USER_AGENT,
            "reddit_to_lemmy_reposter (by u/UltimatePCAddict)",
        )
        .send()
        .await?
        .text()
        .await?;

    let response_json: Value = serde_json::from_str(&response)?;
    let posts_generic = response_json["data"]["children"].clone();
    let response_arr = posts_generic.as_array();

    // Return a Vector of Post struct objects if a successful response has been made
    if response_arr.is_some() {
        let mut posts: Vec<RedditPost> = Vec::new();
        for post in response_arr.unwrap().iter().skip(start_idx) {
            posts.push(RedditPost {
                title: post["data"]["title"].as_str().unwrap().to_string(),
                ups: post["data"]["ups"].as_u64().unwrap(),
                over_18: post["data"]["over_18"].as_bool().unwrap(),
                author: post["data"]["author"].as_str().unwrap().to_string(),
                url: post["data"]["url"].as_str().unwrap().to_string(),
                id: post["data"]["id"].as_str().unwrap().to_string(),
            });
        }
        return Ok(posts);
    } else {
        return Err(format!("Couldn't get posts from {}", subreddit))?;
    }
}
