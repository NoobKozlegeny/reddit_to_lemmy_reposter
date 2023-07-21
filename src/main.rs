use serde_json::{json, Value, Error};

#[tokio::main]
async fn main() { 
    let _ = reddit_get_posts("fosttalicska").await;
}

async fn reddit_get_posts(subreddit: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ...
    let url: String = format!("https://www.reddit.com/r/{}/hot.json", subreddit);
    let client = reqwest::Client::new();
    let posts = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "reddit_to_lemmy_reposter (by u/UltimatePCAddict)")
        .send()
        .await?
        .text()
        .await?;

    let v: Value = serde_json::from_str(&posts)?;
    println!("{}", v["data"]["children"][0]["data"]["title"]);
    Ok(())
}
