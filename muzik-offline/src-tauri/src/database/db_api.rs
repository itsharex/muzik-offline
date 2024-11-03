use super::{db_manager::DbManager, db_struct::ResponseObject};
use crate::components::{album::Album, artist::Artist, genre::Genre, song::Song};
use sled::Tree;

//publicly available api functions
#[tauri::command]
pub async fn get_all_songs_in_db() -> String {
    match DbManager::new() {
        Ok(mut dbm) => {
            let song_tree = match dbm.get_song_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut songs: Vec<Song> = Vec::new();

            let could_retrieve_songs = tokio::task::spawn_blocking(move || {
                for result in song_tree.iter() {
                    match result {
                        Ok((_, song_as_ivec)) => {
                            let song_as_bytes = song_as_ivec.as_ref();
                            let song_as_str = String::from_utf8_lossy(song_as_bytes);
                            match serde_json::from_str::<Song>(&song_as_str.to_string()) {
                                Ok(song) => {
                                    songs.push(song);
                                }
                                Err(_) => {
                                    println!("error converting song from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the song tree");
                        }
                    }
                }

                //convert songs vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: songs,
                }) {
                    Ok(songs_as_json) => {
                        return songs_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_songs {
                Ok(songs_as_json) => {
                    return songs_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

#[tauri::command]
pub async fn get_songs_not_in_vec(uuids_not_to_match: Vec<String>) -> String{
    match DbManager::new() {
        Ok(mut dbm) => {
            let song_tree = match dbm.get_song_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut songs: Vec<Song> = Vec::new();

            let could_retrieve_songs = tokio::task::spawn_blocking(move || {
                for result in song_tree.iter() {
                    match result {
                        Ok((_, song_as_ivec)) => {
                            let song_as_bytes = song_as_ivec.as_ref();
                            let song_as_str = String::from_utf8_lossy(song_as_bytes);
                            match serde_json::from_str::<Song>(&song_as_str.to_string()) {
                                Ok(song) => {
                                    if !uuids_not_to_match.iter().any(|uuid| uuid == &song.uuid.to_string()) {
                                        songs.push(song);
                                    }
                                }
                                Err(_) => {
                                    println!("error converting song from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the song tree");
                        }
                    }
                }

                //convert songs vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: songs,
                }) {
                    Ok(songs_as_json) => {
                        return songs_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_songs {
                Ok(songs_as_json) => {
                    return songs_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

#[tauri::command]
pub async fn get_all_albums() -> String {
    match DbManager::new() {
        Ok(mut dbm) => {
            let album_tree = match dbm.get_album_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut albums: Vec<Album> = Vec::new();

            let could_retrieve_albums = tokio::task::spawn_blocking(move || {
                for result in album_tree.iter() {
                    match result {
                        Ok((_, album_as_ivec)) => {
                            let album_as_bytes = album_as_ivec.as_ref();
                            let album_as_str = String::from_utf8_lossy(album_as_bytes);
                            match serde_json::from_str::<Album>(&album_as_str.to_string()) {
                                Ok(album) => {
                                    albums.push(album);
                                }
                                Err(_) => {
                                    println!("error converting album from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the album tree");
                        }
                    }
                }

                //convert albums vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: albums,
                }) {
                    Ok(albums_as_json) => {
                        return albums_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_albums {
                Ok(albums_as_json) => {
                    return albums_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

#[tauri::command]
pub async fn get_albums_not_in_vec(uuids_not_to_match: Vec<String>) -> String{
    match DbManager::new() {
        Ok(mut dbm) => {
            let album_tree = match dbm.get_album_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut albums: Vec<Album> = Vec::new();

            let could_retrieve_albums = tokio::task::spawn_blocking(move || {
                for result in album_tree.iter() {
                    match result {
                        Ok((_, album_as_ivec)) => {
                            let album_as_bytes = album_as_ivec.as_ref();
                            let album_as_str = String::from_utf8_lossy(album_as_bytes);
                            match serde_json::from_str::<Album>(&album_as_str.to_string()) {
                                Ok(album) => {
                                    if !uuids_not_to_match.iter().any(|uuid| uuid == &album.uuid.to_string()) {
                                        albums.push(album);
                                    }
                                }
                                Err(_) => {
                                    println!("error converting album from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the album tree");
                        }
                    }
                }

                //convert albums vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: albums,
                }) {
                    Ok(albums_as_json) => {
                        return albums_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_albums {
                Ok(albums_as_json) => {
                    return albums_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

#[tauri::command]
pub async fn get_all_artists() -> String {
    match DbManager::new() {
        Ok(mut dbm) => {
            let artist_tree = match dbm.get_artist_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut artists: Vec<Artist> = Vec::new();

            let could_retrieve_artists = tokio::task::spawn_blocking(move || {
                for result in artist_tree.iter() {
                    match result {
                        Ok((_, artist_as_ivec)) => {
                            let artist_as_bytes = artist_as_ivec.as_ref();
                            let artist_as_str = String::from_utf8_lossy(artist_as_bytes);
                            match serde_json::from_str::<Artist>(&artist_as_str.to_string()) {
                                Ok(artist) => {
                                    artists.push(artist);
                                }
                                Err(_) => {
                                    println!("error converting artist from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the artist tree");
                        }
                    }
                }

                //convert artists vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: artists,
                }) {
                    Ok(artists_as_json) => {
                        return artists_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_artists {
                Ok(artists_as_json) => {
                    return artists_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

#[tauri::command]
pub async fn get_artists_not_in_vec(uuids_not_to_match: Vec<String>) -> String{
    match DbManager::new() {
        Ok(mut dbm) => {
            let artist_tree = match dbm.get_artist_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut artists: Vec<Artist> = Vec::new();

            let could_retrieve_artists = tokio::task::spawn_blocking(move || {
                for result in artist_tree.iter() {
                    match result {
                        Ok((_, artist_as_ivec)) => {
                            let artist_as_bytes = artist_as_ivec.as_ref();
                            let artist_as_str = String::from_utf8_lossy(artist_as_bytes);
                            match serde_json::from_str::<Artist>(&artist_as_str.to_string()) {
                                Ok(artist) => {
                                    if !uuids_not_to_match.iter().any(|uuid| uuid == &artist.uuid.to_string()) {
                                        artists.push(artist);
                                    }
                                }
                                Err(_) => {
                                    println!("error converting artist from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the artist tree");
                        }
                    }
                }

                //convert artists vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: artists,
                }) {
                    Ok(artists_as_json) => {
                        return artists_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_artists {
                Ok(artists_as_json) => {
                    return artists_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

#[tauri::command]
pub async fn get_all_genres() -> String {
    match DbManager::new() {
        Ok(mut dbm) => {
            let genre_tree = match dbm.get_genre_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut genres: Vec<Genre> = Vec::new();

            let could_retrieve_genres = tokio::task::spawn_blocking(move || {
                for result in genre_tree.iter() {
                    match result {
                        Ok((_, genre_as_ivec)) => {
                            let genre_as_bytes = genre_as_ivec.as_ref();
                            let genre_as_str = String::from_utf8_lossy(genre_as_bytes);
                            match serde_json::from_str::<Genre>(&genre_as_str.to_string()) {
                                Ok(genre) => {
                                    genres.push(genre);
                                }
                                Err(_) => {
                                    println!("error converting genre from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the genre tree");
                        }
                    }
                }

                //convert genres vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: genres,
                }) {
                    Ok(genres_as_json) => {
                        return genres_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_genres {
                Ok(genres_as_json) => {
                    return genres_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

#[tauri::command]
pub async fn get_genres_not_in_vec(uuids_not_to_match: Vec<String>) -> String{
    match DbManager::new() {
        Ok(mut dbm) => {
            let genre_tree = match dbm.get_genre_tree() {
                Ok(tree) => tree,
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"error\",\"message\":\"{}\",\"data\":[]}}",
                        e
                    ));
                }
            };
            let mut genres: Vec<Genre> = Vec::new();

            let could_retrieve_genres = tokio::task::spawn_blocking(move || {
                for result in genre_tree.iter() {
                    match result {
                        Ok((_, genre_as_ivec)) => {
                            let genre_as_bytes = genre_as_ivec.as_ref();
                            let genre_as_str = String::from_utf8_lossy(genre_as_bytes);
                            match serde_json::from_str::<Genre>(&genre_as_str.to_string()) {
                                Ok(genre) => {
                                    if !uuids_not_to_match.iter().any(|uuid| uuid == &genre.uuid.to_string()) {
                                        genres.push(genre);
                                    }
                                }
                                Err(_) => {
                                    println!("error converting genre from json to struct");
                                }
                            }
                        }
                        Err(_) => {
                            println!("error getting this key from the genre tree");
                        }
                    }
                }

                //convert genres vec to json and return
                match serde_json::to_string(&ResponseObject {
                    status: String::from("success"),
                    message: String::from(""),
                    data: genres,
                }) {
                    Ok(genres_as_json) => {
                        return genres_as_json;
                    }
                    Err(e) => {
                        return String::from(format!(
                            "{{\"status\":\"json parse error\",\"message\":\"{}\",\"data\":[]}}",
                            e.to_string()
                        ));
                    }
                }
            })
            .await;

            match could_retrieve_genres {
                Ok(genres_as_json) => {
                    return genres_as_json;
                }
                Err(e) => {
                    return String::from(format!(
                        "{{\"status\":\"thread error\",\"message\":\"{}\",\"data\":[]}}",
                        e.to_string()
                    ));
                }
            }
        }
        Err(e) => {
            return String::from(format!(
                "{{\"status\":\"lock error\",\"message\":\"{}\",\"data\":[]}}",
                e
            ));
        }
    }
}

pub fn get_image_from_tree(uuid: &str) -> Vec<u8> {
    match DbManager::new() {
        Ok(mut dbm) => {
            let covers_tree = match dbm.get_covers_tree() {
                Ok(tree) => tree,
                Err(_) => {
                    return Vec::new();
                }
            };

            match covers_tree.get(uuid) {
                Ok(Some(cover)) => {
                    return cover.to_vec();
                }
                Ok(None) => {
                    return Vec::new();
                }
                Err(_) => {
                    return Vec::new();
                }
            }
        }
        Err(_) => {
            return Vec::new();
        }
    }
}

pub fn get_song_from_tree(uuid: &str) -> Option<Song> {
    match DbManager::new() {
        Ok(mut dbm) => {
            let song_tree = match dbm.get_song_tree() {
                Ok(tree) => tree,
                Err(_) => {
                    return None;
                }
            };

            match song_tree.get(uuid) {
                Ok(Some(song_as_ivec)) => {
                    let song_as_bytes = song_as_ivec.as_ref();
                    let song_as_str = String::from_utf8_lossy(song_as_bytes);
                    match serde_json::from_str::<Song>(&song_as_str.to_string()) {
                        Ok(song) => {
                            return Some(song);
                        }
                        Err(_) => {
                            return None;
                        }
                    }
                }
                Ok(None) => {
                    return None;
                }
                Err(_) => {
                    return None;
                }
            }
        }
        Err(_) => {
            return None;
        }
    }
}

// new refactored insertion functions

// this function can also overwrite the previous song with the same id
pub fn insert_song_into_tree(song: &Song) {
    match DbManager::new() {
        Ok(mut dbm) => {
            let song_tree = match dbm.get_song_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };

            match serde_json::to_string(&song) {
                Ok(song_as_json) => {
                    match song_tree.insert(song.uuid.to_string(), song_as_json.as_bytes()) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}

pub fn song_exists_in_tree(uuid: &str) -> bool {
    match DbManager::new() {
        Ok(mut dbm) => {
            let song_tree = match dbm.get_song_tree() {
                Ok(tree) => tree,
                Err(_) => {return false;}
            };

            match song_tree.get(uuid) {
                Ok(Some(_)) => {
                    return true;
                }
                Ok(None) => {
                    return false;
                }
                Err(_) => {
                    return false;
                }
            }
        }
        Err(_) => {
            return false;
        }
    }
}

pub fn insert_into_album_tree(song: &Song){
    match DbManager::new() {
        Ok(mut dbm) => {
            let album_tree = match dbm.get_album_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };

            match album_tree.get(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.album.as_bytes()).to_string()) {
                Ok(Some(album_as_ivec)) => {
                    let album_as_bytes = album_as_ivec.as_ref();
                    let album_as_str = String::from_utf8_lossy(album_as_bytes);
                    match serde_json::from_str::<Album>(&album_as_str.to_string()) {
                        Ok(mut album) => {
                            // if album cover is none, set it to some
                            match &album.cover {
                                Some(_) => {}
                                None => {
                                    album.cover = match &song.cover_uuid {
                                        Some(cover) => Some(cover.clone()),
                                        None => None,
                                    };

                                    match serde_json::to_string(&album) {
                                        Ok(album_as_json) => {
                                            match album_tree.insert(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.album.as_bytes()).to_string(), album_as_json.as_bytes()) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                Ok(None) => {
                    let album = Album {
                        key: song.id,
                        // set uuid based on album name
                        uuid: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.album.as_bytes()),
                        cover: match &song.cover_uuid {
                            Some(cover) => Some(cover.clone()),
                            None => None,
                        },
                        title: song.album.clone(),
                    };
                    match serde_json::to_string(&album) {
                        Ok(album_as_json) => {
                            match album_tree.insert(album.uuid.to_string(), album_as_json.as_bytes()) {
                                Ok(_) => {}
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}

pub fn insert_into_artist_tree(song: &Song){
    match DbManager::new() {
        Ok(mut dbm) => {
            let artist_tree = match dbm.get_artist_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };

            match artist_tree.get(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.artist.as_bytes()).to_string()) {
                Ok(Some(artist_as_ivec)) => {
                    let artist_as_bytes = artist_as_ivec.as_ref();
                    let artist_as_str = String::from_utf8_lossy(artist_as_bytes);
                    match serde_json::from_str::<Artist>(&artist_as_str.to_string()) {
                        Ok(mut artist) => {
                            // if artist cover is none, set it to some
                            match &artist.cover {
                                Some(_) => {}
                                None => {
                                    artist.cover = match &song.cover_uuid {
                                        Some(cover) => Some(cover.clone()),
                                        None => None,
                                    };

                                    match serde_json::to_string(&artist) {
                                        Ok(artist_as_json) => {
                                            match artist_tree.insert(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.artist.as_bytes()).to_string(), artist_as_json.as_bytes()) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                Ok(None) => {
                    let artist = Artist {
                        key: song.id,
                        // set uuid based on artist name
                        uuid: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.artist.as_bytes()),
                        cover: match &song.cover_uuid {
                            Some(cover) => Some(cover.clone()),
                            None => None,
                        },
                        artist_name: song.artist.clone(),
                    };
                    match serde_json::to_string(&artist) {
                        Ok(artist_as_json) => {
                            match artist_tree.insert(artist.uuid.to_string(), artist_as_json.as_bytes()) {
                                Ok(_) => {}
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}

pub fn insert_into_genre_tree(song: &Song){
    match DbManager::new() {
        Ok(mut dbm) => {
            let genre_tree = match dbm.get_genre_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };

            match genre_tree.get(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.genre.as_bytes()).to_string()) {
                Ok(Some(genre_as_ivec)) => {
                    let genre_as_bytes = genre_as_ivec.as_ref();
                    let genre_as_str = String::from_utf8_lossy(genre_as_bytes);
                    match serde_json::from_str::<Genre>(&genre_as_str.to_string()) {
                        Ok(mut genre) => {
                            // if genre cover is none, set it to some
                            match &genre.cover {
                                Some(_) => {}
                                None => {
                                    genre.cover = match &song.cover_uuid {
                                        Some(cover) => Some(cover.clone()),
                                        None => None,
                                    };

                                    match serde_json::to_string(&genre) {
                                        Ok(genre_as_json) => {
                                            match genre_tree.insert(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.genre.as_bytes()).to_string(), genre_as_json.as_bytes()) {
                                                Ok(_) => {}
                                                Err(_) => {}
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                Ok(None) => {
                    let genre = Genre {
                        key: song.id,
                        // set uuid based on genre name
                        uuid: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song.genre.as_bytes()),
                        cover: match &song.cover_uuid {
                            Some(cover) => Some(cover.clone()),
                            None => None,
                        },
                        title: song.genre.clone(),
                    };
                    match serde_json::to_string(&genre) {
                        Ok(genre_as_json) => {
                            match genre_tree.insert(genre.uuid.to_string(), genre_as_json.as_bytes()) {
                                Ok(_) => {}
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}

pub fn insert_into_covers_tree(cover: Vec<u8>, song_path: &String) -> uuid::Uuid {
    match DbManager::new() {
        Ok(mut dbm) => {
            let covers_tree = match dbm.get_covers_tree() {
                Ok(tree) => tree,
                Err(_) => {return uuid::Uuid::nil();}
            };

            let cover_uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, song_path.as_bytes());
            match covers_tree.insert(cover_uuid.to_string(), cover) {
                Ok(_) => {
                    return cover_uuid;
                }
                Err(_) => {
                    return uuid::Uuid::nil();
                }
            }
        }
        Err(_) => {
            return uuid::Uuid::nil();
        }
    }
}

pub fn clear_all_trees() {
    match DbManager::new() {
        Ok(mut dbm) => {
            let song_tree = match dbm.get_song_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };
            let album_tree = match dbm.get_album_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };
            let artist_tree = match dbm.get_artist_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };
            let genre_tree = match dbm.get_genre_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };
            let covers_tree = match dbm.get_covers_tree() {
                Ok(tree) => tree,
                Err(_) => {return;}
            };

            clear_tree(&song_tree);
            clear_tree(&album_tree);
            clear_tree(&artist_tree);
            clear_tree(&genre_tree);
            clear_tree(&covers_tree);
        }
        Err(_) => {}
    }
}

fn clear_tree(tree: &Tree) {
    for key in tree.iter().keys() {
        match key {
            Ok(key_as_ivec) => match tree.remove(key_as_ivec.as_ref()) {
                Ok(_) => {}
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
}
