use serde::{Deserialize, Serialize};
use crate::models::common::*;
use crate::models::anime::Anime;
use crate::models::manga::Manga;
use crate::models::search::{Character, Person};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopAnimeResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMangaResponse {
    pub data: Vec<Manga>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopCharactersResponse {
    pub data: Vec<Character>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopPeopleResponse {
    pub data: Vec<Person>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopReview {
    pub mal_id: u32,
    pub url: String,
    #[serde(rename = "type")]
    pub review_type: String,
    pub votes: u32,
    pub date: String,
    pub review: String,
    pub score: u8,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    pub user: TopReviewUser,
    pub entry: TopReviewEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopReviewUser {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopReviewEntry {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopReviewsResponse {
    pub data: Vec<TopReview>,
    pub pagination: Pagination,
} 