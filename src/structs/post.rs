#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub ups: u64,
    pub over_18: bool,
    pub author: String,
    pub url: String,
    pub id: String
}