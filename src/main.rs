use serde_json::Value;

#[tokio::main]
async fn main() { 
    let posts = reddit_get_posts("fosttalicska", 3).await;
    match &posts {
        Ok(value) => println!("Successfully fetched posts!"),
        Err(err) => println!("{}", err),
    }
    
    if let Ok(post) = posts {
        println!("{:#?}", post);
    }
}

async fn reddit_get_posts(subreddit: &str, start_idx: usize) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    // ...
    let url: String = format!("https://www.reddit.com/r/{}/hot.json", subreddit);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "reddit_to_lemmy_reposter (by u/UltimatePCAddict)")
        .send()
        .await?
        .text()
        .await?;

    let mut response_json: Value = serde_json::from_str(&response)?;
    println!("{}", response_json["data"]["children"][start_idx]["data"]["title"]);
    
    let posts = response_json["data"]["children"].as_array_mut();
    if posts.is_some() {
        return Ok(posts.unwrap().clone());
    }
    else {
        return Err(format!("Couldn't get posts from {}", subreddit))?;
    }
}
