use anyhow::{Context, Result};
use image::DynamicImage;
use std::path::PathBuf;

/// The "key" to lookups in a given EmojiStore.
pub struct EmojiSpec {
    pub code: String, // The codepoints which make up the Emoji
    pub platform: String,
}

/// The trait that anything that can store Emojis must implement.
pub trait EmojiStore {
    fn emoji_image(&self, s: EmojiSpec) -> Result<DynamicImage>;
}

/// A simple implementation of an emoji store, which looks in a directory beginning at root
/// followed by a directory with the name of the emoji version, followed by a PNG of the emoji
/// whose filename should be in the format "U+3145 U+E400.png".
/// The full filepath is then <root>/<version>/
pub struct EmojiFS {
    pub root: PathBuf,
}

impl EmojiFS {
    fn as_filepath(&self, e: &EmojiSpec) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.root); // TODO: Remove and parameterise
        path.push(e.platform.to_string());

        // Create filename in format e.g: "U+1F36B U+200D U+1F222"
        // This is surprisingly ugly in stable rust, nightly would add intersperse
        // which'll fix it
        // Some clever iterator zipping could save allocation here but its okay like this
        let mut filename = String::new();
        e.code
            .chars()
            .for_each(|c| filename.push_str(&format!("U+{:X} ", c as u32)));

        // Remove trailing space
        filename.pop();
        filename.push_str(".png");

        path.push(filename);
        path
    }
}

impl EmojiStore for EmojiFS {
    fn emoji_image(&self, s: EmojiSpec) -> Result<DynamicImage> {
        let path = self.as_filepath(&s);
        Ok(image::open(path).context(format!("Cannot find {} in database", s.code))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_filepath() {
        let ej = EmojiSpec {
            code: "\u{1F469}\u{200D}\u{1F393}".to_string(),
            platform: "twitter".to_string(),
        };

        let es = EmojiFS {
            root: PathBuf::from("emoji_scraper"),
        };

        assert_eq!(
            es.as_filepath(&ej),
            PathBuf::from(r"emoji_scraper/twitter/U+1F469 U+200D U+1F393.png")
        );
    }

    #[test]
    fn test_emoji_png() {
        let ej = EmojiSpec {
            code: "\u{1F469}\u{200D}\u{1F393}".to_string(),
            platform: "twitter".to_string(),
        };

        let es = EmojiFS {
            root: PathBuf::from("emoji_scraper"),
        };

        assert!(es.emoji_image(ej).is_ok())
    }
}
