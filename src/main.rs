// Import 3rd party libraries
use std::error::Error;

// Import own modules / Create module tree
use crate::{structs::post::Post, api_callers::reddit::reddit_caller::reddit_get_posts};
pub mod structs;
pub mod api_callers;

#[tokio::main]
async fn main() {
    let posts: Result<Vec<Post>, Box<dyn Error>> = reddit_get_posts("fosttalicska", 3).await;
    match &posts {
        Ok(value) => println!("Successfully fetched posts!"),
        Err(err) => println!("{}", err),
    }

    // Print the number of posts gathered
    println!("{}", posts.as_ref().unwrap().iter().count());
    // Print the posts gathered
    if let Ok(post) = posts {
        println!("{:#?}", post);
    }
}

