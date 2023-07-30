// Import libraries
use std::{error::Error, ops::{Deref, DerefMut}, fs, path::Path};
use once_cell::sync::Lazy;
use reqwest::Client;
use structs::post::RedditPostExt;
use url::Url;

// Import own modules / Create module tree
use crate::{api_callers::{reddit::reddit::reddit_get_posts, lemmy::lemmy::{create_post, create_posts}}, structs::post::RedditPost};
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

    let posted_amount = create_posts("lemmy.basedcount.com".to_string(), "main".to_string(), posts_filtered).await;
    println!("{}", posted_amount);
}

fn reddit_filter_posts(mut posts: Vec<RedditPost>) -> Vec<RedditPost> {
    // Filter posts by upvotes
    posts = posts.iter().filter(|post: &&RedditPost| post.ups >= 200).cloned().collect();

    // Check if the post have already been posted to Lemmy
    posts = posts.reddit_filter_posted(Path::new("posted_to_lemmy.txt")).unwrap().to_owned();

    return posts;
}