use clap::Parser;
// Import libraries
use once_cell::sync::Lazy;
use reqwest::Client;
use std::{
    error::Error,
    fs,
    ops::{Deref, DerefMut},
    path::Path,
};
use structs::post::RedditPostExt;
use url::Url;

// Import own modules / Create module tree
use crate::{
    api_callers::{
        lemmy::lemmy::{create_multiple_post, create_one_post},
        reddit::reddit::reddit_get_posts,
    },
    structs::post::RedditPost,
    command_args::CommandArgs
};
pub mod api_callers;
pub mod statics;
pub mod structs;
pub mod command_args;

#[tokio::main]
async fn main() {
    // Obtaining cmd arguemnts
    let cmd_args = CommandArgs::parse();

    println!("Instance: {}", cmd_args.instance); 
    println!("Community: {}", cmd_args.community); 

    // Get posts from subreddit and print the number of posts
    let posts_result: Result<Vec<RedditPost>, Box<dyn Error>> =
        reddit_get_posts("fosttalicska", 3).await;
    let mut posts: Vec<RedditPost> = Vec::new();
    match &posts_result {
        Ok(value) => {
            println!("Reddit: Successfully fetched posts!");
            posts = value.clone();
        }
        Err(err) => println!("{}", err),
    }

    // Filter these posts by upvote count
    let posts_filtered: Vec<RedditPost> = reddit_filter_posts(posts);

    // Post to Lemmy
    // let posted_amount = create_posts("lemmy.basedcount.com".to_string(), "main".to_string(), posts_filtered).await;
    let posted_amount = create_one_post(
        "lemmy.basedcount.com".to_string(),
        "fosttalicska".to_string(),
        posts_filtered.first().cloned(),
    ).await;
    match posted_amount {
        Some(value) => println!("{}", value),
        None => println!("No new content to post")
    }
}

fn reddit_filter_posts(mut posts: Vec<RedditPost>) -> Vec<RedditPost> {
    // Filter posts by upvotes
    posts = posts
        .iter()
        .filter(|post: &&RedditPost| post.ups >= 200)
        .cloned()
        .collect();

    // Check if the post have already been posted to Lemmy
    posts = posts
        .reddit_filter_posted(Path::new("posted_to_lemmy.txt"))
        .unwrap()
        .to_owned();

    return posts;
}
