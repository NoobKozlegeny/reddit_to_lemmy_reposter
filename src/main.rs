// Import libraries
use std::{error::Error, ops::{Deref, DerefMut}, fs, path::Path};
use once_cell::sync::Lazy;
use reqwest::Client;
use structs::post::RedditPostExt;
use url::Url;

// Import own modules / Create module tree
use crate::{api_callers::{reddit::reddit::reddit_get_posts, lemmy::lemmy::create_post}, structs::post::RedditPost};
pub mod api_callers;
pub mod structs;
pub mod statics;

#[tokio::main]
async fn main() {
    // Get posts from subreddit and print the number of posts
    let posts_result: Result<Vec<RedditPost>, Box<dyn Error>> = reddit_get_posts("fosttalicska", 3).await;
    let mut posts: Vec<RedditPost> = Vec::new();
    match &posts_result {
        Ok(value) => {
            println!("Reddit: Successfully fetched posts!");
            posts = value.clone();
        },
        Err(err) => println!("{}", err),
    }
    println!("{}", posts.len());

    // Filter these posts by upvote count
    let posts_filtered: Vec<RedditPost> = reddit_filter_posts(posts);
    println!("{}", posts_filtered.len());

    // Create a post to Lemmy
    // let post_response = create_post(
    //     "lemmy.basedcount.com".to_owned(),
    //     "main".to_owned(),
    //     "Owo 6".to_owned(),
    //     Some(Url::parse("https://i.pinimg.com/originals/84/fe/56/84fe565a8cfe7627b279d439955ad0a6.jpg").unwrap()), // Some(Url::parse("https://hu.pinterest.com/pin/503769908335656123/").unwrap()),
    //     Some("sniff sniff i-is that a BOY I smell? sniff sniff mmm yes I smell it! BOYSMELL!!!! I smell a boy! W-What is a boy doing here?!?! omygosh what am I gonna do?!?! THERE'S A BOY HERE! I'M FREAKING OUT SO MUCH!!!! calm down calm down and take a nice, deep breathe.... sniff sniff it smells so good! I love boysmell so much!!!! It makes me feel so amazing. I'm getting tingles all over from the delicious boyscent! It's driving me boyCRAZY!!!!!!".to_owned())
    // ).await;
    // println!("{}", post_response.unwrap());
    // print!("...");
}

fn reddit_filter_posts(mut posts: Vec<RedditPost>) -> Vec<RedditPost> {
    // Filter posts by upvotes
    posts = posts.iter().filter(|post: &&RedditPost| post.ups >= 200).cloned().collect();

    // Check if the post have already been posted to Lemmy
    posts = posts.reddit_filter_posted(Path::new("posted_to_lemmy.txt")).unwrap().to_owned();

    return posts;
}