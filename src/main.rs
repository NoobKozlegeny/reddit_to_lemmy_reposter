// Import libraries
use statics::CMD_ARGS;
use std::{
    error::Error,
    fs::{self, create_dir_all},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};
use structs::post::RedditPostExt;

// Import own modules / Create module tree
use crate::{
    api_callers::{
        lemmy::lemmy::{create_multiple_post, create_one_post},
        reddit::reddit::reddit_get_posts,
    },
    structs::post::RedditPost,
};
pub mod api_callers;
pub mod statics;
pub mod structs;
pub mod command_args;

#[tokio::main]
async fn main() {
    // Get posts from subreddit and print the number of posts
    let posts_result: Result<Vec<RedditPost>, Box<dyn Error>> =
        reddit_get_posts(&CMD_ARGS.community[..], 3).await;
    let mut posts: Vec<RedditPost> = Vec::new();
    match &posts_result {
        Ok(value) => {
            println!("Reddit: Successfully fetched posts!");
            posts = value.clone();
        }
        Err(err) => println!("{}", err),
    }

    // Create PATH to access communities txt file. The files are stored in the user's directory
    let path = assemble_communities_file_path().unwrap();

    // Filter these posts by upvote count
    let posts_filtered: Vec<RedditPost> = reddit_filter_posts(posts, path.clone());

    // Post to Lemmy
    // let posted_amount = create_posts("lemmy.basedcount.com".to_string(), "main".to_string(), posts_filtered).await;
    let posted_amount = create_one_post(
        CMD_ARGS.instance.clone(),
        CMD_ARGS.community.clone(),
        posts_filtered.first().cloned(),
        path
    ).await;
    match posted_amount {
        Some(value) => println!("{}", value),
        None => println!("No new content to post")
    }
}

fn reddit_filter_posts(mut posts: Vec<RedditPost>, path: PathBuf) -> Vec<RedditPost> {
    // Filter posts by upvotes
    posts = posts
        .iter()
        .filter(|post: &&RedditPost| post.ups >= 200)
        .cloned()
        .collect();

    // Check if the post have already been posted to Lemmy
    posts = posts
        .reddit_filter_posted(path)
        .unwrap()
        .to_owned();

    return posts;
}

fn assemble_communities_file_path() -> Option<PathBuf> {
    let home_dir = home::home_dir().unwrap().to_str().unwrap().to_owned();
    let path_str = format!("{}/Documents/reddit_to_lemmy_reposter/communities", home_dir);
    let _ = create_dir_all(path_str.clone());
    let path = PathBuf::from(format!("{}/{}.txt", path_str, CMD_ARGS.community));
    
    return Some(path);
}
