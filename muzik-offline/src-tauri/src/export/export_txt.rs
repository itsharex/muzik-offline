use std::{collections::HashSet, sync::{Arc, Mutex}};

use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::{components::song::ExportSong, database::{db_api::get_songs_in_tree, db_manager::DbManager}, utils::general_utils::convert_single_to_double_backward_slash_on_path};

#[tauri::command]
pub async fn export_songs_as_txt(app: tauri::AppHandle, db_manager: State<'_, Arc<Mutex<DbManager>>>, uuids: Vec<String>, fields_to_include: Vec<String>) -> Result<(), String> {
    let songs = get_songs_in_tree(db_manager, uuids);

    let num_titles = songs.len();

    let unique_artists: HashSet<_> = songs.iter().map(|song| &song.artist).collect();
    let num_unique_artists = unique_artists.len();

    let unique_albums: HashSet<_> = songs.iter().map(|song| &song.album).collect();
    let num_unique_albums = unique_albums.len();

    let unique_genres: HashSet<_> = songs.iter().map(|song| &song.genre).collect();
    let num_unique_genres = unique_genres.len();

    let oldest_song = songs.iter().min_by_key(|song| song.year).map(|song| song.name.clone());
    let youngest_song = songs.iter().max_by_key(|song| song.year).map(|song| song.name.clone());

    let longest_song = songs.iter().max_by_key(|song| song.duration_seconds).map(|song| song.name.clone());
    let shortest_song = songs.iter().min_by_key(|song| song.duration_seconds).map(|song| song.name.clone());

    let largest_file = songs.iter().max_by_key(|song| song.file_size).map(|song| song.name.clone());
    let smallest_file = songs.iter().min_by_key(|song| song.file_size).map(|song| song.name.clone());

    let file_types: HashSet<_> = songs.iter().map(|song| &song.file_type).collect();
    
    // convert song to appriopriate json format and add to json array
    let export_songs: Vec<ExportSong> = songs.iter().map(|song| {
        ExportSong {
            title: song.title.clone(),
            artist: song.artist.clone(),
            album: song.album.clone(),
            genre: song.genre.clone(),
            year: song.year,
            duration: song.duration.clone(),
            path: song.path.clone(),
            date_recorded: song.date_recorded.clone(),
            date_released: song.date_released.clone(),
            file_size: song.file_size,
            file_type: song.file_type.clone(),
            overall_bit_rate: song.overall_bit_rate,
            audio_bit_rate: song.audio_bit_rate,
            sample_rate: song.sample_rate,
            bit_depth: song.bit_depth,
            channels: song.channels,
        }
    }).collect();

    let songs_txt = export_songs.iter().map(|song| extract_fields_for_song(&song, &fields_to_include)).collect::<String>();

    let txt = format!("
        Number of Titles: {}\n
        Number of Unique Artists: {}\n
        Number of Unique Albums: {}\n
        Number of Unique Genres: {}\n
        Oldest Song: {}\n
        Youngest Song: {}\n
        Longest Song: {}\n
        Shortest Song: {}\n
        Largest File: {}\n
        Smallest File: {}\n
        File Types: {}\n
        Songs:\n
        {}\n
        {}\n
        {}\n
        ",
        num_titles, num_unique_artists, num_unique_albums, num_unique_genres, 
        oldest_song.unwrap_or("".to_string()), youngest_song.unwrap_or("".to_string()),
        longest_song.unwrap_or("".to_string()), shortest_song.unwrap_or("".to_string()),
        largest_file.unwrap_or("".to_string()), smallest_file.unwrap_or("".to_string()),
        serde_json::to_string(&file_types).unwrap_or("".to_string()),  
        create_divider(fields_to_include.len()), create_header(&fields_to_include), songs_txt
    );

    // then prompt user to save the file with dialog
    let file_path= app.dialog()
        .file()
        .add_filter("", &["txt"])
        .set_file_name("songs.txt")
        .blocking_save_file();

    if let Some(file_path) = file_path {
        match std::fs::write(file_path.to_string(), txt){
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    } else {
        Ok(())
    }
}

fn create_divider(size: usize) -> String {
    let mut divider = String::new();
    for _ in 0..size {
        divider.push_str("+-----------------");
    }
    divider.push_str("+");
    divider
}

fn create_header(fields_to_include: &Vec<String>) -> String {
    let mut header = String::new();
    for field in fields_to_include.iter(){
        match field.as_str(){
            "title" => header.push_str("| Title           "),
            "artist" => header.push_str("| Artist          "),
            "album" => header.push_str("| Album           "),
            "genre" => header.push_str("| Genre           "),
            "year" => header.push_str("| Year            "),
            "duration" => header.push_str("| Duration        "),
            "path" => header.push_str("| Path            "),
            "date_recorded" => header.push_str("| Date Recorded   "),
            "date_released" => header.push_str("| Date Released   "),
            "file_size" => header.push_str("| File Size       "),
            "file_type" => header.push_str("| File Type       "),
            "overall_bit_rate" => header.push_str("| Overall Bit Rate"),
            "audio_bit_rate" => header.push_str("| Audio Bit Rate  "),
            "sample_rate" => header.push_str("| Sample Rate     "),
            "bit_depth" => header.push_str("| Bit Depth       "),
            "channels" => header.push_str("| Channels        "),
            _ => ()
        }
    }
    header.push_str("|");
    header
}

fn fit_or_truncate(s: &str) -> String {
    if s.len() > 17 {
        s.chars().take(14).collect::<String>() + "..."
    } else {
        // add spaces to make it 17 characters
        let mut s = s.to_string();
        for _ in 0..(17 - s.len()) {
            s.push(' ');
        }
        s
    }
}

fn extract_fields_for_song(song: &ExportSong, fields_to_include: &Vec<String>) -> String{
    let mut song_str = String::new();
    song_str.push_str(&create_divider(fields_to_include.len()));
    for field in fields_to_include.iter(){
        match field.as_str(){
            "title" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.title))),
            "artist" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.artist))),
            "album" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.album))),
            "genre" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.genre))),
            "year" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.year.to_string()))),
            "duration" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.duration))),
            "path" => song_str.push_str(&format!("|{}", fit_or_truncate(&convert_single_to_double_backward_slash_on_path(&song.path)))),
            "date_recorded" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.date_recorded))),
            "date_released" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.date_released))),
            "file_size" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.file_size.to_string()))),
            "file_type" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.file_type))),
            "overall_bit_rate" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.overall_bit_rate.to_string()))),
            "audio_bit_rate" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.audio_bit_rate.to_string()))),
            "sample_rate" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.sample_rate.to_string()))),
            "bit_depth" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.bit_depth.to_string()))),
            "channels" => song_str.push_str(&format!("|{}", fit_or_truncate(&song.channels.to_string()))),
            _ => ()
        }
    }
    song_str.push_str("|");
    song_str.push_str(&create_divider(fields_to_include.len()));

    song_str
}