pub mod frontmatter;
use std::fs::DirEntry;
use std::path::PathBuf;

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html, markdown_to_html_with_plugins};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
enum PostError {
    #[error("io error: {0}")]
    IO(String),
}

#[derive(Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub draft: bool,
    pub slug: String,
}

pub struct Post {
    pub frontmatter: Frontmatter,
    pub link: String,
    pub body_html: String,
    pub date: String,
    pub filepath: PathBuf,
}

use tracing::info;

pub fn parse_post(entry: DirEntry) -> color_eyre::eyre::Result<Post> {
    info!("Trying to read post {:?} from file system", entry.path());

    let data = std::fs::read_to_string(entry.path())?;

    let filepath = entry.path();

    let (frontmatter, post_content) = frontmatter::parse(&data)?;

    let slug = frontmatter.slug.clone();

    let mut link = ("/blog/").to_owned();

    link.push_str(&slug);

    info!("Parsing frontmatter for post, {}", slug);

    let mut plugins = comrak::ComrakPlugins::default();

    let adapter = SyntectAdapter::new("base16-ocean.dark");

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    info!("Rendering markdown to html");

    let html =
        markdown_to_html_with_plugins(&post_content, &comrak::ComrakOptions::default(), &plugins);

    let date = frontmatter.date.clone();

    Ok(Post {
        frontmatter,
        link,
        body_html: html,
        date,
        filepath,
    })
}
