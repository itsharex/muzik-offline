//this file should contain commands and functions that facilitate the refreshing
//or rather rechecking of the file paths the user gave to see if any new music has been loaded
//or if any music has been deleted and changing the library accordingly.


use super::metadata_retriever::{decode_directories, get_songs_in_path};

#[tauri::command]
pub async fn refresh_paths(paths_as_json_array: String, compress_image_option: bool) -> Result<String, String> {   
    let paths_as_vec = decode_directories(&paths_as_json_array);

    let mut song_id: i32 = 0;
    for path in &paths_as_vec {
        get_songs_in_path(
            &path,
            &mut song_id,
            &compress_image_option,
            true,
        )
        .await;
    }

    return Ok("{\"status\":\"success\"}".into());
}