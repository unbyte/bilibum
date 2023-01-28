use crate::client::Video;
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::{Input, MultiSelect};
use std::io;

pub struct Prompt {
    theme: Box<dyn Theme>,
}

impl Prompt {
    fn capture_video_post_id(id_or_url: impl AsRef<str>) -> Option<String> {
        lazy_regex::regex_captures!(r#"BV([0-9a-zA-Z]+)"#, id_or_url.as_ref())
            .map(|(_, id)| id.to_string())
    }

    pub fn new() -> Self {
        Self {
            theme: Box::new(ColorfulTheme::default()),
        }
    }

    pub fn ask_video_post_id(&self) -> io::Result<String> {
        let id_or_url = Input::with_theme(self.theme.as_ref())
            .with_prompt("id")
            .allow_empty(false)
            .validate_with(|input: &String| match Self::capture_video_post_id(input) {
                None => Err("not a valid id or url"),
                Some(_) => Ok(()),
            })
            .interact()?;
        Ok(Self::capture_video_post_id(&id_or_url).unwrap())
    }

    pub fn select_video_ids(&self, videos: &[Video]) -> io::Result<Vec<u64>> {
        Ok(MultiSelect::with_theme(self.theme.as_ref())
            .with_prompt("select videos")
            .items(&videos)
            .interact()?
            .iter()
            .map(|&idx| videos[idx].id)
            .collect())
    }
}
