use std::{collections::HashSet, sync::{Arc, Mutex}};

use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::{components::song::ExportSong, database::{db_api::get_songs_in_tree, db_manager::DbManager}, utils::general_utils::convert_single_to_double_backward_slash_on_path};

#[tauri::command]
pub async fn export_songs_as_xml(app: tauri::AppHandle, db_manager: State<'_, Arc<Mutex<DbManager>>>, uuids: Vec<String>, fields_to_include: Vec<String>) -> Result<(), String> {
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

    let songs_xml = export_songs.iter().map(|song| extract_fields_for_song(&song, &fields_to_include)).collect::<String>();

    let xml = format!("
        <numTitles>{}</numTitles>\n
        <numUniqueArtists>{}</numUniqueArtists>\n
        <numUniqueAlbums>{}</numUniqueAlbums>\n
        <numUniqueGenres>{}</numUniqueGenres>\n
        <oldestSong>{}</oldestSong>\n
        <youngestSong>{}</youngestSong>\n
        <longestSong>{}</longestSong>\n
        <shortestSong>{}</shortestSong>\n
        <largestFile>{}</largestFile>\n
        <smallestFile>{}</smallestFile>\n
        <fileTypes>{}</fileTypes>\n
        <songs>\n
            {}
        </songs>\n", 
        num_titles, num_unique_artists, num_unique_albums, num_unique_genres, 
        oldest_song.unwrap_or("".to_string()), youngest_song.unwrap_or("".to_string()),
        longest_song.unwrap_or("".to_string()), shortest_song.unwrap_or("".to_string()),
        largest_file.unwrap_or("".to_string()), smallest_file.unwrap_or("".to_string()),
        serde_json::to_string(&file_types).unwrap_or("".to_string()), songs_xml
    );

    // then prompt user to save the file with dialog
    let file_path= app.dialog()
        .file()
        .add_filter("", &["xml"])
        .set_file_name("songs.xml")
        .blocking_save_file();

    if let Some(file_path) = file_path {
        match std::fs::write(file_path.to_string(), xml){
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    } else {
        Ok(())
    }
}

fn extract_fields_for_song(song: &ExportSong, fields_to_include: &Vec<String>) -> String{
    let mut song_str = String::new();
    song_str.push_str("<song>\n");
    for field in fields_to_include.iter(){
        match field.as_str(){
            "title" => song_str.push_str(&format!("<title>{}</title>\n", song.title)),
            "artist" => song_str.push_str(&format!("<artist>{}</artist>\n", song.artist)),
            "album" => song_str.push_str(&format!("<album>{}</album>\n", song.album)),
            "genre" => song_str.push_str(&format!("<genre>{}</genre>\n", song.genre)),
            "year" => song_str.push_str(&format!("<year>{}</year>\n", song.year)),
            "duration" => song_str.push_str(&format!("<duration>{}</duration>\n", song.duration)),
            "path" => song_str.push_str(&format!("<path>{}</path>\n", convert_single_to_double_backward_slash_on_path(&song.path))),
            "date_recorded" => song_str.push_str(&format!("<dateRecorded>{}</dateRecorded>\n", song.date_recorded)),
            "date_released" => song_str.push_str(&format!("<dateReleased>{}</dateReleased>\n", song.date_released)),
            "file_size" => song_str.push_str(&format!("<fileSize>{}</fileSize>\n", song.file_size)),
            "file_type" => song_str.push_str(&format!("<fileType>{}</fileType>\n", song.file_type)),
            "overall_bit_rate" => song_str.push_str(&format!("<overallBitRate>{}</overallBitRate>\n", song.overall_bit_rate)),
            "audio_bit_rate" => song_str.push_str(&format!("<audioBitRate>{}</audioBitRate>\n", song.audio_bit_rate)),
            "sample_rate" => song_str.push_str(&format!("<sampleRate>{}</sampleRate>\n", song.sample_rate)),
            "bit_depth" => song_str.push_str(&format!("<bitDepth>{}</bitDepth>\n", song.bit_depth)),
            "channels" => song_str.push_str(&format!("<channels>{}</channels>\n", song.channels)),
            _ => ()
        }
    }
    
    song_str.push_str("</song>\n");
    song_str
}