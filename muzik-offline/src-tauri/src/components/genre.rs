use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Genre {
    pub key: i32,
    pub uuid: Uuid,
    pub cover: Option<String>,
    pub title: String,
}
