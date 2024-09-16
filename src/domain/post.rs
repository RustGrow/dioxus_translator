use chrono::prelude::*;

pub enum PostLanguage {
    English(Post, String),
    Spanish(Post, String),
    German(Post, String),
    Arabic(Post, String),
}

// impl PostLanguage {
//     pub fn new() -> Self {}
// }

pub struct Post {
    pub title: String,
    pub slug: String,
    pub text: String,
    pub time: String,
}

impl Post {
    pub fn new(title: String, slug: String, text: String) -> Self {
        let now = Utc::now().format("%Y-%m-%d").to_string();
        Post {
            title,
            slug,
            text,
            time: now,
        }
    }
}
