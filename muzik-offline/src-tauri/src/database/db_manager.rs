use dirs::home_dir;
use sled::{Db, Tree};
use std::path::PathBuf;

pub struct DbManager {
    pub db: Db
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

        Ok(DbManager {
            db,
        })
    }

    pub fn get_song_tree(&mut self) -> Result<Tree, String> {
        self.db.open_tree(b"songs").map_err(|e| e.to_string())
    }

    pub fn get_album_tree(&mut self) -> Result<Tree, String> {
        self.db.open_tree(b"albums").map_err(|e| e.to_string())
    }

    pub fn get_artist_tree(&mut self) -> Result<Tree, String> {
        self.db.open_tree(b"artists").map_err(|e| e.to_string())
    }

    pub fn get_genre_tree(&mut self) -> Result<Tree, String> {
        self.db.open_tree(b"genres").map_err(|e| e.to_string())
    }

    pub fn get_covers_tree(&mut self) -> Result<Tree, String> {
        self.db.open_tree(b"covers").map_err(|e| e.to_string())
    }

    pub fn get_thumbnail_tree(&mut self) -> Result<Tree, String> {
        self.db.open_tree(b"thumbnails").map_err(|e| e.to_string())
    }

    pub fn get_wallpaper_tree(&mut self) -> Result<Tree, String> {
        self.db.open_tree(b"wallpapers").map_err(|e| e.to_string())
    }
}
