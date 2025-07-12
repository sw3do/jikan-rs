use crate::models::common::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Vec<String>,
    pub favorites: u32,
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAnime {
    pub role: String,
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
pub struct CharacterManga {
    pub role: String,
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
pub struct CharacterVoiceActor {
    pub language: String,
    pub person: PersonMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPicture {
    pub jpg: Image,
    pub webp: Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterResponse {
    pub data: Character,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterFullResponse {
    pub data: Character,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAnimeResponse {
    pub data: Vec<CharacterAnime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMangaResponse {
    pub data: Vec<CharacterManga>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterVoiceActorsResponse {
    pub data: Vec<CharacterVoiceActor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPicturesResponse {
    pub data: Vec<CharacterPicture>,
}
