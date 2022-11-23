use std::env::current_exe;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
enum AssetError {
    #[error("Could not find 'blog' folder")]
    IO,
}

use tracing::info;

pub fn locate_blog() -> color_eyre::eyre::Result<PathBuf> {
    info!("Locating blog assets");

    let mut root_path: PathBuf = match (option_env!("CARGO_MANIFEST_DIR")) {
        Some(p) => Path::new(p).to_path_buf(),
        None => current_exe()?,
    };

    let blog_path = root_path.join("blog");
    if blog_path.is_dir() {
        info!("Successfully located blog assets at {:?}", blog_path);
        return Ok(blog_path);
    } else {
        return Err(AssetError::IO)?;
    }
}
