use std::fs::DirEntry;

use crate::assets::locate_blog;
use crate::posts::{self, Post};

pub struct State {
    pub posts: Vec<Post>,
}

impl State {
    pub fn init() -> color_eyre::Result<Self> {
        let blog_path = locate_blog()?;
        let dir_entries = std::fs::read_dir(blog_path)?.collect::<Result<Vec<DirEntry>, _>>()?;

        let parsed_posts = dir_entries
            .into_iter()
            .map(|p| posts::parse_post(p))
            .collect::<Result<Vec<Post>, _>>()?;

        Ok(Self {
            posts: parsed_posts,
        })
    }
}
