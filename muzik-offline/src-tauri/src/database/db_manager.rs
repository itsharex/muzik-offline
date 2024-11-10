use dirs::home_dir;
use sled::{Db, Tree};
use std::sync::RwLock;
use std::path::PathBuf;

pub struct DbManager {
    pub song_tree: RwLock<Tree>,
    pub album_tree: RwLock<Tree>,
    pub artist_tree: RwLock<Tree>,
    pub genre_tree: RwLock<Tree>,
    pub covers_tree: RwLock<Tree>,
    pub thumbnails_tree: RwLock<Tree>,
    pub wallpapers_tree: RwLock<Tree>,
    pub playlists_tree: RwLock<Tree>,
}

impl DbManager {
    pub fn new() -> Result<Self, String> {
        let mut db_path = PathBuf::new();
        match home_dir() {
            Some(path) => db_path.push(path),
            None => return Err("Could not find home directory".to_string()),
        }
        db_path.push("muzik-offline-local-data");
        db_path.push("db");
        let db: Db = sled::open(db_path).map_err(|e| e.to_string())?;
        let song_tree = db.open_tree(b"songs").map_err(|e| e.to_string())?;
        let album_tree = db.open_tree(b"albums").map_err(|e| e.to_string())?;
        let artist_tree = db.open_tree(b"artists").map_err(|e| e.to_string())?;
        let genre_tree = db.open_tree(b"genres").map_err(|e| e.to_string())?;
        let covers_tree = db.open_tree(b"covers").map_err(|e| e.to_string())?;
        let thumbnails_tree = db.open_tree(b"thumbnails").map_err(|e| e.to_string())?;
        let wallpapers_tree = db.open_tree(b"wallpapers").map_err(|e| e.to_string())?;
        let playlists_tree = db.open_tree(b"playlists").map_err(|e| e.to_string())?;

        Ok(DbManager {
            song_tree: RwLock::new(song_tree),
            album_tree: RwLock::new(album_tree),
            artist_tree: RwLock::new(artist_tree),
            genre_tree: RwLock::new(genre_tree),
            covers_tree: RwLock::new(covers_tree),
            thumbnails_tree: RwLock::new(thumbnails_tree),
            wallpapers_tree: RwLock::new(wallpapers_tree),
            playlists_tree: RwLock::new(playlists_tree),
        })
    }
}
