use serde::{Deserialize, Serialize};
use crate::models::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEpisode {
    pub mal_id: Option<u32>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub premium: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPromo {
    pub title: Option<String>,
    pub trailer: Option<Trailer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPagination {
    pub last_visible_page: Option<u32>,
    pub has_next_page: Option<bool>,
    pub current_page: Option<u32>,
    pub items: Option<WatchPaginationItems>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPaginationItems {
    pub count: Option<u32>,
    pub total: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEpisodesResponse {
    pub data: Vec<WatchEpisode>,
    pub pagination: Option<WatchPagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPromosResponse {
    pub data: Vec<WatchPromo>,
    pub pagination: Option<WatchPagination>,
} 