use crate::models::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub images: Images,
    pub members: u32,
    pub category: String,
    pub created: Option<DateTime<Utc>>,
    pub access: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubMember {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubStaff {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubRelation {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubResponse {
    pub data: Club,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubMembersResponse {
    pub data: Vec<ClubMember>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubStaffResponse {
    pub data: Vec<ClubStaff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubRelationsResponse {
    pub data: Vec<ClubRelation>,
}
