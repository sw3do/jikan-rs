use serde::{Deserialize, Serialize};
use crate::models::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub mal_id: u32,
    pub url: String,
    pub website_url: Option<String>,
    pub images: Images,
    pub name: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub alternate_names: Vec<String>,
    pub birthday: Option<String>,
    pub favorites: u32,
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAnime {
    pub position: String,
    pub anime: AnimeMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonManga {
    pub position: String,
    pub manga: MangaMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonVoiceActing {
    pub role: String,
    pub anime: AnimeMeta,
    pub character: CharacterMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonPicture {
    pub jpg: Image,
    pub webp: Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonResponse {
    pub data: Person,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonFullResponse {
    pub data: Person,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAnimeResponse {
    pub data: Vec<PersonAnime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonMangaResponse {
    pub data: Vec<PersonManga>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonVoiceActingResponse {
    pub data: Vec<PersonVoiceActing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonPicturesResponse {
    pub data: Vec<PersonPicture>,
} 