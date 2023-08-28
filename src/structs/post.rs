use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct RedditPost {
    pub title: String,
    pub ups: u64,
    pub over_18: bool,
    pub author: String,
    pub url: String,
    pub id: String,
}

pub trait RedditPostExt {
    fn reddit_filter_posted(&mut self, path: PathBuf) -> Result<&mut Self, Box<dyn Error>>;
}

impl RedditPostExt for Vec<RedditPost> {
    fn reddit_filter_posted(&mut self, path: PathBuf) -> Result<&mut Self, Box<dyn Error>> {
        // Read file and create a reader
        // println!("Current Path: {:?}", env::current_dir());
        // println!("File Path: {:?}", path.as_os_str());
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();

        let reader = BufReader::new(file);

        // Remove already posted posts from Vec
        for (i, line_result) in reader.lines().enumerate() {
            let line = line_result.unwrap();
            let matched_post_idx = self.iter().position(|post: &RedditPost| post.id == line);
            if matched_post_idx.is_some() {
                self.remove(matched_post_idx.unwrap());
            }
        }

        return Ok(self);
    }
}
