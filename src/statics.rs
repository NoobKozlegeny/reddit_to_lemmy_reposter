use clap::Parser;
use hyper::{http::HeaderValue, HeaderMap};
use once_cell::sync::Lazy;
use reqwest::Client;

use crate::structs::command_args::CommandArgs;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::new()
});

pub static HEADERS: Lazy<HeaderMap> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        "User-Agent",
        HeaderValue::from_str("reddit_to_lemmy_reposter (by u/PrivateNoob@sopuli.xyz)").unwrap(),
    );
    headers
});

// Obtaining cmd arguemnts
pub static CMD_ARGS: Lazy<CommandArgs> = Lazy::new(|| {
    CommandArgs::parse()
});