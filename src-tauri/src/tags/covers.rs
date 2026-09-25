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

#[cfg(test)]
mod tests {
    use super::*;
    use lofty::file::TaggedFileExt;
    use lofty::picture::MimeType;
    use lofty::prelude::{ItemKey, TagExt};

    const COVER_PNG: &str = "./tests/images/cover.png";
    const COVER_REPLACEMENT_PNG: &str = "./tests/images/cover_replacement.png";
    const EXTRA_PICTURE_JPG: &str = "./tests/images/extra_picture.jpg";

    fn copy_fixture(temp_dir: &tempfile::TempDir) -> std::path::PathBuf {
        let song_path = temp_dir.path().join("some_song.mp3");
        std::fs::copy(
            "./tests/music_libraries/one_file_with_tags/some_song.mp3",
            &song_path,
        )
        .unwrap();
        song_path
    }

    fn front_pictures_of(song_path: &Path) -> Vec<Picture> {
        lofty::read_from_path(song_path)
            .unwrap()
            .primary_tag()
            .unwrap()
            .pictures()
            .iter()
            .filter(|picture| picture.pic_type() == PictureType::CoverFront)
            .cloned()
            .collect()
    }

    fn picture_from_file(path: &str, pic_type: PictureType) -> Picture {
        let mut picture =
            Picture::from_reader(&mut BufReader::new(File::open(path).unwrap())).unwrap();
        picture.set_pic_type(pic_type);
        picture
    }

    fn seed_picture(song_path: &Path, picture: Picture) {
        let mut tagged_file = lofty::read_from_path(song_path).unwrap();
        tagged_file
            .primary_tag_mut()
            .unwrap()
            .push_picture(picture);
        tagged_file
            .save_to_path(song_path, WriteOptions::default())
            .unwrap();
    }

    #[test]
    fn test_write_cover_sets_front_cover() {
        let temp_dir = tempfile::tempdir().unwrap();
        let song_path = copy_fixture(&temp_dir);
        let expected_bytes = std::fs::read(COVER_PNG).unwrap();

        write_cover_to_file(&song_path, Some(Path::new(COVER_PNG))).unwrap();

        let pictures = front_pictures_of(&song_path);
        assert_eq!(pictures.len(), 1);
        assert_eq!(pictures[0].mime_type(), Some(&MimeType::Png));
        assert_eq!(pictures[0].data(), expected_bytes.as_slice());
    }

    #[test]
    fn test_write_cover_replaces_existing_cover() {
        let temp_dir = tempfile::tempdir().unwrap();
        let song_path = copy_fixture(&temp_dir);
        let expected_bytes = std::fs::read(COVER_REPLACEMENT_PNG).unwrap();

        write_cover_to_file(&song_path, Some(Path::new(COVER_PNG))).unwrap();
        write_cover_to_file(&song_path, Some(Path::new(COVER_REPLACEMENT_PNG))).unwrap();

        let pictures = front_pictures_of(&song_path);
        assert_eq!(pictures.len(), 1, "replacing must not stack covers");
        assert_eq!(pictures[0].data(), expected_bytes.as_slice());
    }

    #[test]
    fn test_write_cover_keeps_non_front_pictures() {
        let temp_dir = tempfile::tempdir().unwrap();
        let song_path = copy_fixture(&temp_dir);
        seed_picture(
            &song_path,
            picture_from_file(EXTRA_PICTURE_JPG, PictureType::CoverBack),
        );

        write_cover_to_file(&song_path, Some(Path::new(COVER_PNG))).unwrap();

        let pictures = lofty::read_from_path(&song_path)
            .unwrap()
            .primary_tag()
            .unwrap()
            .pictures()
            .to_vec();
        assert_eq!(pictures.len(), 2, "a back cover should survive a front cover write");
        assert_eq!(
            pictures
                .iter()
                .filter(|picture| picture.pic_type() == PictureType::CoverFront)
                .count(),
            1
        );
        assert_eq!(
            pictures
                .iter()
                .filter(|picture| picture.pic_type() == PictureType::CoverBack)
                .count(),
            1
        );
    }

    #[test]
    fn test_write_cover_removal_clears_all_pictures() {
        let temp_dir = tempfile::tempdir().unwrap();
        let song_path = copy_fixture(&temp_dir);
        seed_picture(
            &song_path,
            picture_from_file(COVER_PNG, PictureType::CoverFront),
        );
        seed_picture(
            &song_path,
            picture_from_file(EXTRA_PICTURE_JPG, PictureType::CoverBack),
        );

        write_cover_to_file(&song_path, None).unwrap();

        let tagged_file = lofty::read_from_path(&song_path).unwrap();
        assert!(
            tagged_file.primary_tag().unwrap().pictures().is_empty(),
            "removal should clear every picture so nothing is displayed"
        );
    }

    #[test]
    fn test_write_cover_preserves_other_tags() {
        let temp_dir = tempfile::tempdir().unwrap();
        let song_path = copy_fixture(&temp_dir);

        let track_number_before = lofty::read_from_path(&song_path)
            .unwrap()
            .primary_tag()
            .unwrap()
            .get_string(&ItemKey::TrackNumber)
            .map(str::to_owned);
        assert!(
            track_number_before.is_some(),
            "fixture should carry a track number tag"
        );

        write_cover_to_file(&song_path, Some(Path::new(COVER_PNG))).unwrap();

        let reloaded = lofty::read_from_path(&song_path).unwrap();
        assert_eq!(
            reloaded
                .primary_tag()
                .unwrap()
                .get_string(&ItemKey::TrackNumber),
            track_number_before.as_deref()
        );
    }
}