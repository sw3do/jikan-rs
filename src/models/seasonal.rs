use serde::{Deserialize, Serialize};
use crate::models::common::*;
use crate::models::anime::Anime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalAnimeResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
    pub season_name: Option<String>,
    pub season_year: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonNowResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
    pub season_name: Option<String>,
    pub season_year: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonUpcomingResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonArchiveResponse {
    pub data: Vec<SeasonArchiveYear>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonArchiveYear {
    pub year: u16,
    pub seasons: Vec<String>,
} 