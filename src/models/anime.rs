use crate::models::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub trailer: Option<Trailer>,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: bool,
    pub aired: DateRange,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f64>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u16>,
    pub broadcast: Option<Broadcast>,
    pub producers: Vec<Producer>,
    pub licensors: Vec<Licensor>,
    pub studios: Vec<Studio>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<Genre>,
    pub themes: Vec<Theme>,
    pub demographics: Vec<Demographic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCharacter {
    pub character: CharacterMeta,
    pub role: String,
    pub voice_actors: Vec<VoiceActor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceActor {
    pub person: PersonMeta,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStaff {
    pub person: PersonMeta,
    pub positions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisode {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    pub aired: Option<DateTime<Utc>>,
    pub score: Option<f64>,
    pub filler: bool,
    pub recap: bool,
    pub forum_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeNews {
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
pub struct AnimeVideo {
    pub promo: Vec<PromoVideo>,
    pub episodes: Vec<EpisodeVideo>,
    pub music_videos: Vec<MusicVideo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoVideo {
    pub title: String,
    pub trailer: Trailer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeVideo {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub episode: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicVideo {
    pub title: String,
    pub video: VideoMeta,
    pub meta: MusicVideoMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMeta {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
    pub images: Option<TrailerImages>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicVideoMeta {
    pub title: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimePicture {
    pub jpg: Image,
    pub webp: Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStatistics {
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
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
pub struct AnimeMoreInfo {
    pub moreinfo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeRecommendation {
    pub entry: AnimeMeta,
    pub url: String,
    pub votes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeMeta {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeUserUpdate {
    pub user: UserMeta,
    pub score: Option<u8>,
    pub status: String,
    pub episodes_seen: Option<u32>,
    pub episodes_total: Option<u32>,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeta {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeReview {
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
    pub episodes_watched: Option<u32>,
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
pub struct AnimeRelation {
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
pub struct AnimeTheme {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeExternal {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStreaming {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeForum {
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
pub struct AnimeResponse {
    pub data: Anime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeFullResponse {
    pub data: Anime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCharactersResponse {
    pub data: Vec<AnimeCharacter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStaffResponse {
    pub data: Vec<AnimeStaff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisodesResponse {
    pub data: Vec<AnimeEpisode>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisodeResponse {
    pub data: AnimeEpisode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeNewsResponse {
    pub data: Vec<AnimeNews>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeForumResponse {
    pub data: Vec<AnimeForum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeVideosResponse {
    pub data: AnimeVideo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeVideosEpisodesResponse {
    pub data: Vec<EpisodeVideo>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimePicturesResponse {
    pub data: Vec<AnimePicture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStatisticsResponse {
    pub data: AnimeStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeMoreInfoResponse {
    pub data: AnimeMoreInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeRecommendationsResponse {
    pub data: Vec<AnimeRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeUserUpdatesResponse {
    pub data: Vec<AnimeUserUpdate>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeReviewsResponse {
    pub data: Vec<AnimeReview>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeRelationsResponse {
    pub data: Vec<AnimeRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeThemesResponse {
    pub data: AnimeTheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeExternalResponse {
    pub data: Vec<AnimeExternal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStreamingResponse {
    pub data: Vec<AnimeStreaming>,
}
