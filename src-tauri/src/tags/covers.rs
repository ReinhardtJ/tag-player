use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::{Context, Result};
use lofty::config::WriteOptions;
use lofty::picture::{Picture, PictureType};
use lofty::prelude::AudioFile;
use crate::tags::writing_tags::get_or_create_primary_tag;

pub fn write_cover_to_file(song_path: &Path, cover_path: Option<&Path>) -> Result<()> {
    let mut tagged_file = lofty::read_from_path(song_path)
        .with_context(|| format!("Failed to read audio file: {}", song_path.display()))?;

    let tag = get_or_create_primary_tag(&mut tagged_file);

    match cover_path {
        Some(path) => {
            let mut picture = Picture::from_reader(&mut BufReader::new(File::open(path)?))?;
            picture.set_pic_type(PictureType::CoverFront);
            tag.remove_picture_type(PictureType::CoverFront);
            tag.push_picture(picture);
        },
        None => {
            while tag.picture_count() > 0 {
                tag.remove_picture(0);
            }
        }

    }
    tagged_file.save_to_path(song_path, WriteOptions::default())?;
    Ok(())
}