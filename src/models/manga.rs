use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    #[serde(rename = "type")]
    pub manga_type: Option<String>,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: Option<String>,
    pub publishing: bool,
    pub published: DateRange,
    pub score: Option<f64>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub authors: Vec<Author>,
    pub serializations: Vec<Serialization>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<Genre>,
    pub themes: Vec<Theme>,
    pub demographics: Vec<Demographic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub mal_id: u32,
    pub url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub author_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Serialization {
    pub mal_id: u32,
    pub url: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaCharacter {
    pub character: CharacterMeta,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaNews {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub author_username: String,
    pub author_url: String,
    pub forum_url: String,
    pub images: Images,
    pub comments: u32,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaPicture {
    pub jpg: Image,
    pub webp: Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaStatistics {
    pub reading: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_read: u32,
    pub total: u32,
    pub scores: Vec<ScoreStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreStats {
    pub score: u8,
    pub votes: u32,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaMoreInfo {
    pub moreinfo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaRecommendation {
    pub entry: MangaMeta,
    pub url: String,
    pub votes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaUserUpdate {
    pub user: UserMeta,
    pub score: Option<u8>,
    pub status: String,
    pub chapters_read: Option<u32>,
    pub chapters_total: Option<u32>,
    pub volumes_read: Option<u32>,
    pub volumes_total: Option<u32>,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeta {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaReview {
    pub mal_id: u32,
    pub url: String,
    #[serde(rename = "type")]
    pub review_type: String,
    pub reactions: ReviewReactions,
    pub date: DateTime<Utc>,
    pub review: String,
    pub score: u8,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    pub chapters_read: Option<u32>,
    pub user: UserMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewReactions {
    pub overall: u32,
    pub nice: u32,
    pub love_it: u32,
    pub funny: u32,
    pub confusing: u32,
    pub informative: u32,
    pub well_written: u32,
    pub creative: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaRelation {
    pub relation: String,
    pub entry: Vec<RelationEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationEntry {
    pub mal_id: u32,
    pub url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entry_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaExternal {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaForum {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub author_username: String,
    pub author_url: String,
    pub comments: u32,
    pub last_comment: LastComment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastComment {
    pub url: String,
    pub author_username: String,
    pub author_url: String,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaResponse {
    pub data: Manga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaFullResponse {
    pub data: Manga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaCharactersResponse {
    pub data: Vec<MangaCharacter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaNewsResponse {
    pub data: Vec<MangaNews>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaForumResponse {
    pub data: Vec<MangaForum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaPicturesResponse {
    pub data: Vec<MangaPicture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaStatisticsResponse {
    pub data: MangaStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaMoreInfoResponse {
    pub data: MangaMoreInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaRecommendationsResponse {
    pub data: Vec<MangaRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaUserUpdatesResponse {
    pub data: Vec<MangaUserUpdate>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaReviewsResponse {
    pub data: Vec<MangaReview>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaRelationsResponse {
    pub data: Vec<MangaRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaExternalResponse {
    pub data: Vec<MangaExternal>,
} 