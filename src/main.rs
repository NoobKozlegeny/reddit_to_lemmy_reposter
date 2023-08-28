use log::warn;
// Import libraries
use statics::CMD_ARGS;
use std::{error::Error, fs::create_dir_all, path::PathBuf, env::consts::OS};

// Import own modules / Create module tree
use crate::{
    api_callers::{lemmy::lemmy::create_one_post, reddit::reddit::reddit_get_posts},
    structs::post::*,
};
pub mod api_callers;
pub mod statics;
pub mod structs;

#[tokio::main]
async fn main() {
    // Initalize logger
    env_logger::init();

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
    let path = assemble_communities_file_path();

    // Filter these posts by upvote count
    let posts_filtered: Vec<RedditPost> =
        reddit_filter_posts(posts, path.clone(), CMD_ARGS.min_ups, CMD_ARGS.min_ups);

    // Post to Lemmy
    let posted_amount = create_one_post(
        CMD_ARGS.instance.clone(),
        CMD_ARGS.community.clone(),
        posts_filtered.first().cloned(),
        path,
    )
    .await;
    match posted_amount {
        Some(value) => println!("{}", value),
        None => println!("No new content to post"),
    }
}

fn reddit_filter_posts(
    mut posts: Vec<RedditPost>,
    path: PathBuf,
    min_ups: Option<u64>,
    max_ups: Option<u64>,
) -> Vec<RedditPost> {
    // Filter posts by upvotes
    posts = posts
        .iter()
        .filter(|post: &&RedditPost| {
            post.ups >= min_ups.unwrap_or(0) && post.ups <= max_ups.unwrap_or(u64::MAX)
        })
        .cloned()
        .collect();

    // Check if the post have already been posted to Lemmy
    posts = posts.reddit_filter_posted(path).unwrap().to_owned();

    return posts;
}

/// Creates the path where the script will store already posted posts's IDs in an OS agnostic way
fn assemble_communities_file_path() -> PathBuf {
    // Closure which gets the home directory path + Documents. If not then it creates a path somewhere in root/C:
    let create_home_or_base_path = || {
        let home_path = home::home_dir(); //.unwrap().to_str().unwrap().to_owned();

        if home_path.is_some() {
            return Some(home_path?.join("Documents"));
        }
        else {
            // Creates path based on which OS is running
            // These are UNIX: Linux, MacOS, BSD, etc?
            if cfg!(unix) {
                return Some(PathBuf::from("/root/usr/local/etc"));
            }
            else if cfg!(windows) {
                return Some(PathBuf::from("/Program Files"));
            }
            else {
                warn!("Couldn't detect your operating system. The script will store the posted posts's ids somewhere top level (like int root, C:)");
                return None;
            }
        }
    };

    // Assembles full path and creates directories which are missing
    let path_join = format!("reddit_to_lemmy_reposter/communities/{}.txt", CMD_ARGS.community);
    let path = create_home_or_base_path().unwrap().join(path_join);
    let _ = create_dir_all(path.clone());

    return path;
}
