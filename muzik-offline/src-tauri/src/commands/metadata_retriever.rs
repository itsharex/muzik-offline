use crate::components::song::Song;
use crate::database::db_api::{clear_all_trees, insert_into_album_tree, insert_into_artist_tree, insert_into_covers_tree, insert_into_genre_tree, insert_song_into_tree};
use crate::utils::general_utils::{
    duration_to_string, extract_file_name, resize_and_compress_image,
};
use crate::utils::general_utils::is_media_file;
use id3::TagLike;
use lofty::Accessor;
use lofty::TaggedFileExt;
use lofty::{AudioFile, Probe};
use serde_json::Value;
use std::path::Path;

#[tauri::command]
pub async fn get_all_songs(
    paths_as_json_array: String,
    compress_image_option: bool,
) -> Result<String, String> {
    clear_all_trees();
    
    let paths_as_vec = decode_directories(&paths_as_json_array);

    let mut song_id: i32 = 0;
    for path in &paths_as_vec {
        get_songs_in_path(
            &path,
            &mut song_id,
            &compress_image_option,
        )
        .await;
    }

    return Ok("{\"status\":\"success\"}".into());
}

pub fn decode_directories(paths_as_json: &str) -> Vec<String> {
    let parsed_json: Result<Value, serde_json::Error> = serde_json::from_str(paths_as_json);

    match parsed_json {
        Ok(parsed_json) => {
            // Ensure the parsed JSON is an array
            if let Value::Array(array) = parsed_json {
                // Convert each element to a String
                let string_vec: Vec<String> = array
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();

                return string_vec;
            } else {
                return Vec::new();
            }
        }
        Err(_) => return Vec::new(),
    }
}

pub async fn get_songs_in_path(
    dir_path: &str,
    song_id: &mut i32,
    compress_image_option: &bool,
){

    match tokio::fs::read_dir(dir_path).await {
        Ok(mut paths) => {
            while let Ok(Some(entry)) = paths.next_entry().await {
                match entry.path().to_str() {
                    Some(full_path) => {
                        match is_media_file(full_path) {
                            Ok(true) => {}
                            Ok(false) => {
                                continue;
                            }
                            Err(_) => {
                                continue;
                            }
                        }

                        if let Ok(song_data) =
                            read_from_path(full_path, song_id, compress_image_option)
                        {
                            insert_song_into_tree(&song_data);
                            insert_into_album_tree(&song_data);
                            insert_into_artist_tree(&song_data);
                            insert_into_genre_tree(&song_data);
                        } else if let Ok(song_data) =
                            lofty_read_from_path(full_path, song_id, compress_image_option)
                        {
                            insert_song_into_tree(&song_data);
                            insert_into_album_tree(&song_data);
                            insert_into_artist_tree(&song_data);
                            insert_into_genre_tree(&song_data);
                        } else {
                        }
                    }
                    None => {}
                }
            }
        }
        Err(_) => {}
    }
}

fn lofty_read_from_path(
    path: &str,
    song_id: &mut i32,
    compress_image_option: &bool,
) -> Result<Song, Box<dyn std::error::Error>> {
    let tagged_file = lofty::read_from_path(path)?;
    *song_id += 1;

    let mut song_meta_data = Song {
        id: *song_id,
        uuid: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, path.as_bytes()),
        title: String::from(""),
        name: String::from(""),
        artist: String::from(""),
        album: String::from(""),
        genre: String::from(""),
        year: 0,
        duration: String::from(""),
        duration_seconds: 0,
        path: String::from(""),
        cover_uuid: None,
        date_recorded: String::from(""),
        date_released: String::from(""),
        file_size: 0,
        file_type: String::from(""),
        overall_bit_rate: 0,
        audio_bit_rate: 0,
        sample_rate: 0,
        bit_depth: 0,
        channels: 0,
    };

    match tagged_file.first_tag() {
        Some(tag) => {
            set_title_lofty(tag, &mut song_meta_data);
            set_name(&path, &mut song_meta_data);
            set_artist_lofty(tag, &mut song_meta_data);
            set_album_lofty(tag, &mut song_meta_data);
            set_genre_lofty(tag, &mut song_meta_data);
            set_year_lofty(tag, &mut song_meta_data);
            set_duration_bit_rate_sample_rate_bit_depth_channels(&path, &mut song_meta_data);
            set_path(&path, &mut song_meta_data);
            set_cover_lofty(tag, &mut song_meta_data, compress_image_option, &path);
            set_date_recorded_lofty(tag, &mut song_meta_data);
            set_date_released_lofty(tag, &mut song_meta_data);
            set_file_size(&path, &mut song_meta_data);
            set_file_extension(&path, &mut song_meta_data);
        }
        None => {
            song_meta_data.title = String::from("Unknown Title");
            set_name(&path, &mut song_meta_data);
            song_meta_data.artist = String::from("Unknown Artist");
            song_meta_data.album = String::from("Unknown Album");
            song_meta_data.genre = String::from("Unknown Genre");
            song_meta_data.year = 0;
            set_duration_bit_rate_sample_rate_bit_depth_channels(&path, &mut song_meta_data);
            set_path(&path, &mut song_meta_data);
            song_meta_data.cover_uuid = None;
            song_meta_data.date_recorded = String::from("Unknown date recorded");
            song_meta_data.date_released = String::from("Unknown date recorded");
            set_file_size(&path, &mut song_meta_data);
            set_file_extension(&path, &mut song_meta_data);
        }
    }

    Ok(song_meta_data)
}

fn read_from_path(
    path: &str,
    song_id: &mut i32,
    compress_image_option: &bool,
) -> Result<Song, Box<dyn std::error::Error>> {
    let tag = match id3::Tag::read_from_path(path) {
        Ok(tag) => tag,
        Err(id3::Error {
            kind: id3::ErrorKind::NoTag,
            ..
        }) => id3::Tag::new(),
        Err(err) => return Err(Box::new(err)),
    };

    *song_id += 1;

    let mut song_meta_data = Song {
        id: *song_id,
        uuid: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, path.as_bytes()),
        title: String::from(""),
        name: String::from(""),
        artist: String::from(""),
        album: String::from(""),
        genre: String::from(""),
        year: 0,
        duration: String::from(""),
        duration_seconds: 0,
        path: String::from(""),
        cover_uuid: None,
        date_recorded: String::from(""),
        date_released: String::from(""),
        file_size: 0,
        file_type: String::from(""),
        overall_bit_rate: 0,
        audio_bit_rate: 0,
        sample_rate: 0,
        bit_depth: 0,
        channels: 0,
    };

    set_title_id3(&tag, &mut song_meta_data);
    set_name(&path, &mut song_meta_data);
    set_artist_id3(&tag, &mut song_meta_data);
    set_album_id3(&tag, &mut song_meta_data);
    set_genre_id3(&tag, &mut song_meta_data);
    set_year_id3(&tag, &mut song_meta_data);
    set_duration_bit_rate_sample_rate_bit_depth_channels(&path, &mut song_meta_data);
    set_path(&path, &mut song_meta_data);
    set_cover_id3(&tag, &mut song_meta_data, compress_image_option, &path);
    set_date_recorded_id3(&tag, &mut song_meta_data);
    set_date_released_id3(&tag, &mut song_meta_data);
    set_file_size(&path, &mut song_meta_data);
    set_file_extension(&path, &mut song_meta_data);

    Ok(song_meta_data)
}

fn set_title_id3(tag: &id3::Tag, song_meta_data: &mut Song) {
    //TITLE
    if let Some(title) = tag.title() {
        song_meta_data.title = title.to_owned();
    } else {
        song_meta_data.title = String::from("Unknown Title");
    }
}

fn set_title_lofty(tag: &lofty::Tag, song_meta_data: &mut Song) {
    //TITLE
    if let Some(title) = tag.title() {
        song_meta_data.title = title.to_string();
    } else {
        song_meta_data.title = String::from("Unknown Title");
    }
}

fn set_name(path: &str, song_meta_data: &mut Song) {
    //NAME
    song_meta_data.name = extract_file_name(&path);
}

fn set_artist_id3(tag: &id3::Tag, song_meta_data: &mut Song) {
    //ARTIST
    if let Some(artist) = tag.artist() {
        song_meta_data.artist = artist.to_owned();
    } else {
        song_meta_data.artist = String::from("Unknown Artist");
    }
}

fn set_artist_lofty(tag: &lofty::Tag, song_meta_data: &mut Song) {
    //ARTIST
    if let Some(artist) = tag.artist() {
        song_meta_data.artist = artist.to_string();
    } else {
        song_meta_data.artist = String::from("Unknown Artist");
    }
}

fn set_album_id3(tag: &id3::Tag, song_meta_data: &mut Song) {
    //ALBUM
    if let Some(album) = tag.album() {
        song_meta_data.album = album.to_owned();
    } else {
        song_meta_data.album = String::from("Unknown Album");
    }
}

fn set_album_lofty(tag: &lofty::Tag, song_meta_data: &mut Song) {
    //ALBUM
    if let Some(album) = tag.album() {
        song_meta_data.album = album.to_string();
    } else {
        song_meta_data.album = String::from("Unknown Album");
    }
}

fn set_genre_id3(tag: &id3::Tag, song_meta_data: &mut Song) {
    //GENRE
    if let Some(genre) = tag.genre() {
        song_meta_data.genre = genre.to_owned();
    } else {
        song_meta_data.genre = String::from("Unknown Genre");
    }
}

fn set_genre_lofty(tag: &lofty::Tag, song_meta_data: &mut Song) {
    //GENRE
    if let Some(genre) = tag.genre() {
        song_meta_data.genre = genre.to_string();
    } else {
        song_meta_data.genre = String::from("Unknown Genre");
    }
}

fn set_year_id3(tag: &id3::Tag, song_meta_data: &mut Song) {
    //YEAR
    if let Some(year) = tag.year() {
        song_meta_data.year = year as u32;
    } else {
        song_meta_data.year = 0;
    }
}

fn set_year_lofty(tag: &lofty::Tag, song_meta_data: &mut Song) {
    //YEAR
    if let Some(year) = tag.year() {
        song_meta_data.year = year;
    } else {
        song_meta_data.year = 0;
    }
}

fn set_duration_bit_rate_sample_rate_bit_depth_channels(path: &str, song_meta_data: &mut Song) {
    //DURATION, OVERALL BIT RATE, AUDIO BIT RATE, SAMPLE RATE, BIT DEPTH, CHANNELS
    match Probe::open(path) {
        Ok(probed) => match probed.read() {
            Ok(tagged_file) => {
                song_meta_data.duration_seconds = tagged_file.properties().duration().as_secs();
                song_meta_data.duration =
                    duration_to_string(&tagged_file.properties().duration().as_secs());
                song_meta_data.overall_bit_rate =
                    tagged_file.properties().overall_bitrate().unwrap_or(0);
                song_meta_data.audio_bit_rate =
                    tagged_file.properties().audio_bitrate().unwrap_or(0);
                song_meta_data.sample_rate = tagged_file.properties().sample_rate().unwrap_or(0);
                song_meta_data.bit_depth = tagged_file.properties().bit_depth().unwrap_or(0);
                song_meta_data.channels = tagged_file.properties().channels().unwrap_or(0);
            }
            Err(_) => {
                song_meta_data.duration_seconds = 0;
                song_meta_data.duration = String::from("00:00");
                song_meta_data.overall_bit_rate = 0;
                song_meta_data.audio_bit_rate = 0;
                song_meta_data.sample_rate = 0;
                song_meta_data.bit_depth = 0;
                song_meta_data.channels = 0;
            }
        },
        Err(_) => {
            song_meta_data.duration_seconds = 0;
            song_meta_data.duration = String::from("00:00");
            song_meta_data.overall_bit_rate = 0;
            song_meta_data.audio_bit_rate = 0;
            song_meta_data.sample_rate = 0;
            song_meta_data.bit_depth = 0;
            song_meta_data.channels = 0;
        }
    }
}

fn set_path(path: &str, song_meta_data: &mut Song) {
    //PATH
    song_meta_data.path = path.to_owned();
}

fn set_cover_id3(tag: &id3::Tag, song_meta_data: &mut Song, compress_image_option: &bool, path: &str) {
    //COVER
    if let Some(cover) = tag.pictures().next() {
        let picture_as_num = cover.data.to_owned();
        match compress_image_option {
            true => {
                //we want the image to be compressed to have speed improvements
                //the image resides in picture_as_num as a Vec<u8>
                //compression code goes here
                match resize_and_compress_image(&picture_as_num, &250) {
                    Some(compressed_image) => {
                        song_meta_data.cover_uuid = match insert_into_covers_tree(compressed_image, &path.to_owned()) {
                            uuid if uuid == uuid::Uuid::nil() => None,
                            uuid => Some(uuid.to_string()),
                        }
                    }
                    None => {
                        song_meta_data.cover_uuid = match insert_into_covers_tree(picture_as_num, &path.to_owned()) {
                            uuid if uuid == uuid::Uuid::nil() => None,
                            uuid => Some(uuid.to_string()),
                        }
                    }
                }
            }
            false => {
                song_meta_data.cover_uuid = match insert_into_covers_tree(picture_as_num, &path.to_owned()) {
                    uuid if uuid == uuid::Uuid::nil() => None,
                    uuid => Some(uuid.to_string()),
                }
            }
        }
    } else {
        song_meta_data.cover_uuid = None;
    }
}

fn set_cover_lofty(tag: &lofty::Tag, song_meta_data: &mut Song, compress_image_option: &bool, path: &str) {
    //COVER
    let pictures = tag.pictures();

    if pictures.len() == 0 {
        song_meta_data.cover_uuid = None;
    } else {
        for picture in pictures {
            let picture_as_num = picture.data().to_owned();
            match compress_image_option {
                true => {
                    //we want the image to be compressed to have speed improvements
                    //the image resides in picture_as_num as a Vec<u8>
                    //compression code goes here
                    match resize_and_compress_image(&picture_as_num, &250) {
                        Some(compressed_image) => {
                            song_meta_data.cover_uuid = match insert_into_covers_tree(compressed_image, &path.to_owned()) {
                                uuid if uuid == uuid::Uuid::nil() => None,
                                uuid => Some(uuid.to_string()),
                            }
                        }
                        None => {
                            song_meta_data.cover_uuid = match insert_into_covers_tree(picture_as_num, &path.to_owned()) {
                                uuid if uuid == uuid::Uuid::nil() => None,
                                uuid => Some(uuid.to_string()),
                            }
                        }
                    }
                }
                false => {
                    song_meta_data.cover_uuid = match insert_into_covers_tree(picture_as_num, &path.to_owned()) {
                        uuid if uuid == uuid::Uuid::nil() => None,
                        uuid => Some(uuid.to_string()),
                    }
                }
            }
            break;
        }
    }
}

fn set_date_recorded_id3(tag: &id3::Tag, song_meta_data: &mut Song) {
    //DATE RECORDED
    //"YYYY-MM-DD-HH-MM-SS"
    if let Some(date_recorded) = tag.date_recorded() {
        song_meta_data.date_recorded = format!(
            "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}",
            date_recorded.year,
            date_recorded.month.unwrap_or(0),
            date_recorded.day.unwrap_or(0),
            date_recorded.hour.unwrap_or(0),
            date_recorded.minute.unwrap_or(0),
            date_recorded.second.unwrap_or(0)
        );
    } else {
        song_meta_data.date_recorded = String::from("Unknown date recorded");
    }
}

fn set_date_recorded_lofty(_audio_file: &lofty::Tag, song_meta_data: &mut Song) {
    //needs re-implementation
    //lofty provides no way to access this
    song_meta_data.date_recorded = String::from("Unknown date recorded");
}

fn set_date_released_id3(tag: &id3::Tag, song_meta_data: &mut Song) {
    //DATE RELEASED
    //"YYYY-MM-DD-HH-MM-SS"
    if let Some(date_released) = tag.date_released() {
        song_meta_data.date_released = format!(
            "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}",
            date_released.year,
            date_released.month.unwrap_or(0),
            date_released.day.unwrap_or(0),
            date_released.hour.unwrap_or(0),
            date_released.minute.unwrap_or(0),
            date_released.second.unwrap_or(0)
        );
    } else {
        song_meta_data.date_released = String::from("Unknown date recorded");
    }
}

fn set_date_released_lofty(_audio_file: &lofty::Tag, song_meta_data: &mut Song) {
    //needs re-implementation
    //lofty provides no way to access this
    song_meta_data.date_released = String::from("Unknown date recorded");
}

fn set_file_size(path: &str, song_meta_data: &mut Song) {
    //SIZE
    let real_path = Path::new(&path);
    match std::fs::metadata(&real_path) {
        Ok(metadata) => {
            song_meta_data.file_size = metadata.len();
        }
        Err(_) => {
            song_meta_data.file_size = 0;
        }
    }
}

fn set_file_extension(path: &str, song_meta_data: &mut Song) {
    //FILE TYPE
    let real_path = Path::new(&path);
    match real_path.extension() {
        Some(wrapped_extension) => match wrapped_extension.to_str() {
            Some(unwrapped_extension) => {
                song_meta_data.file_type = unwrapped_extension.to_string();
            }
            None => {
                song_meta_data.file_type = String::from("Unknown file type");
            }
        },
        None => {
            song_meta_data.file_type = String::from("Unknown file type");
        }
    }
}
