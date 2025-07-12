use crate::models::anime::Anime;
use crate::models::common::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleDay {
    pub monday: Option<Vec<Anime>>,
    pub tuesday: Option<Vec<Anime>>,
    pub wednesday: Option<Vec<Anime>>,
    pub thursday: Option<Vec<Anime>>,
    pub friday: Option<Vec<Anime>>,
    pub saturday: Option<Vec<Anime>>,
    pub sunday: Option<Vec<Anime>>,
    pub other: Option<Vec<Anime>>,
    pub unknown: Option<Vec<Anime>>,
}
