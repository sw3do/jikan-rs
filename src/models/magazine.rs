use crate::models::common::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Magazine {
    pub mal_id: u32,
    pub url: String,
    pub name: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagazineResponse {
    pub data: Magazine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagazinesResponse {
    pub data: Vec<Magazine>,
    pub pagination: Pagination,
}
