/// This code was borrowed from @fasterthanlime.
use color_eyre::eyre::Result;

use gray_matter::engine::YAML;
use gray_matter::Matter;

pub use super::Frontmatter as Data;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("EOF while parsing frontmatter")]
    EOF,
    #[error("Error parsing yaml")]
    Yaml,
}

pub fn parse(input: &str) -> Result<(Data, String)> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse_with_struct::<Data>(input).ok_or(Error::Yaml)?;
    Ok((result.data, result.content))
}
