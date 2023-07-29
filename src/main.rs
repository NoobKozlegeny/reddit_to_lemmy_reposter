// Import libraries
use std::error::Error;
use once_cell::sync::Lazy;
use reqwest::Client;
use url::Url;

// Import own modules / Create module tree
use crate::{api_callers::{reddit::reddit_caller::reddit_get_posts, lemmy::lemmy::create_post}, structs::post::Post};
pub mod api_callers;
pub mod structs;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    return Client::new();
});

#[tokio::main]
async fn main() {
    // Get posts from subreddit
    let posts: Result<Vec<Post>, Box<dyn Error>> = reddit_get_posts("fosttalicska", 3).await;
    match &posts {
        Ok(_) => println!("Successfully fetched posts!"),
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