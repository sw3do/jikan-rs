use crate::models::anime::Anime;
use crate::models::common::*;
use crate::models::manga::Manga;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeSearchResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaSearchResponse {
    pub data: Vec<Manga>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSearchResponse {
    pub data: Vec<Character>,
    pub pagination: Pagination,
}

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
pub struct PersonSearchResponse {
    pub data: Vec<Person>,
    pub pagination: Pagination,
}

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
pub struct UserSearchResponse {
    pub data: Vec<UserProfile>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub url: String,
    pub images: Images,
    pub last_online: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubSearchResponse {
    pub data: Vec<Club>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub images: Images,
    pub members: u32,
    pub category: String,
    pub created: Option<String>,
    #[serde(rename = "type")]
    pub club_type: String,
}
