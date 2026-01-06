use std::path::Path;
use tag_player_lib::music_library::*;

#[test]
fn test_gather_music_library_non_existent_folder() {
    let library = read_music_library(Path::new("/nonexistent"));
    assert_eq!(library.songs.len(), 0);
    assert_eq!(library.errors.len(), 1); // expected IO error
}

#[test]
fn test_gather_music_library_empty_folder() {
    let library = read_music_library(Path::new("./tests/music_libraries/empty"));
    assert_eq!(library.songs.len(), 0);
    assert_eq!(library.errors.len(), 0);
}
#[test]
fn test_gather_music_library_one_file_with_tags() {
    let library = read_music_library(Path::new("./tests/music_libraries/one_file_with_tags"));
    assert_eq!(library.songs.len(), 1);
    assert_eq!(library.errors.len(), 0);
    if let Some(song) = library.songs.first() {
        assert_eq!(song.name, "some_song.mp3");
        assert_eq!(song.tags.track_number, Some(1));
    } else {
    }
}
