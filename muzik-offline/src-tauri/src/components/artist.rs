use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub key: i32,
    pub uuid: Uuid,
    pub cover: Option<String>,
    pub artist_name: String,
}
