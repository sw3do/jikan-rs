use serde::{Deserialize, Serialize};
use crate::models::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreResponse {
    pub data: Vec<Genre>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeGenresResponse {
    pub data: Vec<Genre>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaGenresResponse {
    pub data: Vec<Genre>,
} 