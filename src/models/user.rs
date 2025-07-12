use crate::models::common::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub url: String,
    pub images: Images,
    pub last_online: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub location: Option<String>,
    pub joined: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub anime: UserAnimeStatistics,
    pub manga: UserMangaStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnimeStatistics {
    pub days_watched: f64,
    pub mean_score: f64,
    pub watching: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_watch: u32,
    pub total_entries: u32,
    pub rewatched: u32,
    pub episodes_watched: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMangaStatistics {
    pub days_read: f64,
    pub mean_score: f64,
    pub reading: u32,
    pub completed: u32,
    pub on_hold: u32,
    pub dropped: u32,
    pub plan_to_read: u32,
    pub total_entries: u32,
    pub reread: u32,
    pub chapters_read: u32,
    pub volumes_read: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub data: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatisticsResponse {
    pub data: UserStatistics,
}
